# **leftpad-rs**
Why bother making your own left padding function in like 2 minutes when you can instead download this awesome library that gives you the most **🔥 blazingly fast 🔥** left-padding code out there. Want to use TypeScript? You can! thanks to the amazing work the folks over at [napi-rs](https://napi.rs/) do!

Install it like so:
```bash 
npm install @leftpad-rs/core
```

To use it, simply write:
```js
import { leftpad } from "@leftpad-rs/core"

console.log( leftpad("Hello World!", 5) );
//     Hello World!
```

The function takes in three arguments:
- **original (required)**: The string to leftpad
- **length (required)**: The number of spaces to leftpad by
- pattern: The pattern to repeat by, default is a whitespace " ", can be any string
