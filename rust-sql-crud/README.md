# App Structure

```
.
├── Cargo.lock
├── Cargo.toml
└── src
    ├── main.rs          // main server file
    ├── routes           // global route which will serve the main.rs nested route eg /users
    │   └── mod.rs       // imports every habdler, route, models from users folder
    └── users
        ├── handlers.    // business logic for all routes
		│      ├── mod.rs // exports
		│      ├── all_users.rs 
		│      ├── create.rs
		│      ├── delete.rs
		│      ├── root_path.rs 
        ├── mod.rs      // exports handlers, models and routes to parent/other dirs
        ├── models.rs   // pub structs are defined which will be used in other dirs
        └── routes.rs   // path defined for "/users" which will serve sub paths
			            // eg: /create, /getallusers, /update/:id etc
```

### Run
Check if there is a cargo error first: ```cargo check```

Start live server using: ```` cargo watch -q -c -x run````