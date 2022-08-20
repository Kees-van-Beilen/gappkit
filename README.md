# gappkit
just a test 😏
still in alpha so not production ready


goals: 
- write ones runs everywhere
- macro based ui language i.e. 
```rust
Window::new(())
    .ui(
        View!{
            HStack!{
                Text!("I'm to the left")
                VStack!{
                    Text!("I'm on the top right")
                    Text!("I'm on the bottom right")
                }
            }
        }
    )
```
- respecting target system's design guidelines