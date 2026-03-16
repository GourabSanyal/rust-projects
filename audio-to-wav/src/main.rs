use anyhow::{Context, Result};
use hound::{SampleFormat, WavSpec, WavWriter};
use rubato::{FftFixedIn, Resampler};
use std::fs;
use std::path::Path;
use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

const TARGET_SAMPLE_RATE: u32 = 16000;

fn convert_to_wav(input_path: &Path, output_path: &Path) -> Result<()> {
    // --- 1. Open and probe the input file ---
    let file = fs::File::open(input_path)
        .with_context(|| format!("Could not open input file: {}", input_path.display()))?;

    let mss = MediaSourceStream::new(Box::new(file), Default::default());

    let mut hint = Hint::new();
    if let Some(ext) = input_path.extension().and_then(|e| e.to_str()) {
        hint.with_extension(ext);
    }

    let probed = symphonia::default::get_probe()
        .format(
            &hint,
            mss,
            &FormatOptions::default(),
            &MetadataOptions::default(),
        )
        .context("Unsupported format or could not probe file")?;

    let mut format = probed.format;

    let track = format
        .default_track()
        .context("No default audio track found")?;

    let codec_params = track.codec_params.clone();
    let track_id = track.id;

    let source_sample_rate = codec_params
        .sample_rate
        .context("Could not read source sample rate")?;

    let source_channels = codec_params
        .channels
        .context("Could not read channel info")?
        .count();

    println!("  Source sample rate : {} Hz", source_sample_rate);
    println!("  Source channels    : {}", source_channels);

    // --- 2. Create decoder ---
    let mut decoder = symphonia::default::get_codecs()
        .make(&codec_params, &DecoderOptions::default())
        .context("Could not create decoder")?;

    // --- 3. Decode all packets into interleaved f32 samples ---
    let mut raw_samples: Vec<f32> = Vec::new();

    loop {
        let packet = match format.next_packet() {
            Ok(p) => p,
            Err(symphonia::core::errors::Error::IoError(_)) => break,
            Err(symphonia::core::errors::Error::ResetRequired) => break,
            Err(e) => return Err(e.into()),
        };

        if packet.track_id() != track_id {
            continue;
        }

        match decoder.decode(&packet) {
            Ok(decoded) => {
                let mut sample_buf =
                    SampleBuffer::<f32>::new(decoded.capacity() as u64, *decoded.spec());
                sample_buf.copy_interleaved_ref(decoded);
                raw_samples.extend_from_slice(sample_buf.samples());
            }
            Err(symphonia::core::errors::Error::DecodeError(_)) => continue,
            Err(e) => return Err(e.into()),
        }
    }

    println!("  Decoded samples    : {}", raw_samples.len());

    // --- 4. Mix down to mono ---
    let mono_samples: Vec<f32> = if source_channels == 1 {
        raw_samples
    } else {
        raw_samples
            .chunks(source_channels)
            .map(|frame| frame.iter().sum::<f32>() / source_channels as f32)
            .collect()
    };

    // --- 5. Resample to 16kHz using rubato ---
    let resampled: Vec<f32> = if source_sample_rate == TARGET_SAMPLE_RATE {
        mono_samples
    } else {
        println!(
            "  Resampling         : {} Hz → {} Hz",
            source_sample_rate, TARGET_SAMPLE_RATE
        );

        let chunk_size = 1024;
        let mut resampler = FftFixedIn::<f32>::new(
            source_sample_rate as usize,
            TARGET_SAMPLE_RATE as usize,
            chunk_size,
            2,
            1, // mono
        )
        .context("Could not create resampler")?;

        let mut output_samples: Vec<f32> = Vec::new();
        let mut pos = 0;

        while pos + chunk_size <= mono_samples.len() {
            let chunk = vec![mono_samples[pos..pos + chunk_size].to_vec()];
            let out = resampler.process(&chunk, None)?;
            output_samples.extend_from_slice(&out[0]);
            pos += chunk_size;
        }

        // flush remaining samples
        if pos < mono_samples.len() {
            let mut last_chunk = mono_samples[pos..].to_vec();
            last_chunk.resize(chunk_size, 0.0);
            let chunk = vec![last_chunk];
            let out = resampler.process(&chunk, None)?;
            output_samples.extend_from_slice(&out[0]);
        }

        output_samples
    };

    // --- 6. Write WAV (16-bit PCM, 16kHz, mono) ---
    let spec = WavSpec {
        channels: 1,
        sample_rate: TARGET_SAMPLE_RATE,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };

    let mut writer =
        WavWriter::create(output_path, spec).context("Could not create output WAV file")?;

    for sample in &resampled {
        let clamped = sample.clamp(-1.0, 1.0);
        let pcm = (clamped * i16::MAX as f32) as i16;
        writer.write_sample(pcm)?;
    }

    writer.finalize()?;

    println!(
        "  Output samples     : {} ({:.2}s)",
        resampled.len(),
        resampled.len() as f32 / TARGET_SAMPLE_RATE as f32
    );

    Ok(())
}

fn main() -> Result<()> {
    let input_dir = Path::new("input");
    let output_dir = Path::new("output");

    fs::create_dir_all(output_dir)?;

    let entries: Vec<_> = fs::read_dir(input_dir)
        .context("Could not read input/ folder")?
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .and_then(|x| x.to_str())
                .map(|ext| matches!(ext.to_lowercase().as_str(), "mp3" | "m4a" | "aac" | "flac" | "ogg" | "wav"))
                .unwrap_or(false)
        })
        .collect();

    if entries.is_empty() {
        println!("No supported audio files found in input/");
        println!("Supported: mp3, m4a, aac, flac, ogg, wav");
        return Ok(());
    }

    for entry in entries {
        let input_path = entry.path();
        let stem = input_path.file_stem().unwrap_or_default();
        let output_path = output_dir.join(format!("{}_16khz_mono.wav", stem.to_string_lossy()));

        println!("\n🎵 Converting: {}", input_path.display());
        match convert_to_wav(&input_path, &output_path) {
            Ok(_) => println!("✅ Done → {}", output_path.display()),
            Err(e) => println!("❌ Failed: {e}"),
        }
    }

    Ok(())
}
