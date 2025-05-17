use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
#[schema(example = json!({
    "id": "post-1",
    "title": "The Future of Rust in Web Development",
    "date": "2025-04-10",
    "tags": ["Rust", "Web Development", "Backend"],
    "excerpt": "Exploring how Rust is changing the landscape of web development with its performance and safety guarantees.",
    "content": "# The Future of Rust in Web Development\n\nAs web applications become more complex..."
}))]
pub struct Post {
    pub id: String,
    pub title: String,
    pub date: String,
    pub tags: Vec<String>,
    pub excerpt: String,
    pub content: String,
}

impl Post {
    pub fn new(
        title: String,
        date: String,
        tags: Vec<String>,
        excerpt: String,
        content: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            date,
            tags,
            excerpt,
            content,
        }
    }
}

// Mock data for development
pub fn get_mock_posts() -> Vec<Post> {
    vec![
        Post {
            id: "post-1".to_string(),
            title: "The Future of Rust in Web Development".to_string(),
            date: "2025-04-10".to_string(),
            tags: vec![
                "Rust".to_string(),
                "Web Development".to_string(),
                "Backend".to_string(),
            ],
            excerpt: "Exploring how Rust is changing the landscape of web development with its performance and safety guarantees.".to_string(),
            content: r#"
# The Future of Rust in Web Development

As web applications become more complex and users demand better performance, Rust has emerged as a compelling alternative to traditional web development languages. In this post, I'll explore how Rust is changing the landscape of web development.

## Performance Benefits

Rust's zero-cost abstractions and memory safety guarantees make it an excellent choice for performance-critical applications. Unlike garbage-collected languages, Rust provides memory safety without the overhead of a garbage collector.

Some key performance benefits include:

- Near-native performance
- Predictable memory usage
- No garbage collection pauses
- Excellent concurrency support

## Growing Ecosystem

While Rust's web ecosystem is still maturing, it's growing rapidly. Frameworks like Actix Web, Rocket, and Warp provide excellent performance and developer experience.

### Popular Rust Web Frameworks

1. **Actix Web** - Ultra-fast, feature-rich framework
2. **Rocket** - Focus on ergonomics and developer experience
3. **Warp** - Composable, middleware-based approach
4. **Axum** - From the creators of Tokio, designed for ergonomics

## Real-World Adoption

Companies like Discord, Cloudflare, and Dropbox have adopted Rust for performance-critical components of their web infrastructure, demonstrating its viability for production use.

## Conclusion

While Rust may not completely replace languages like JavaScript, Python, or Go in web development, it's carving out an important niche for performance-critical applications and systems programming aspects of web services.

The future looks bright for Rust in web development, especially as WebAssembly continues to gain adoption, opening new possibilities for using Rust in both frontend and backend development.
            "#.to_string(),
        },
        Post {
            id: "post-2".to_string(),
            title: "Building Retro-Tech UIs for Modern Applications".to_string(),
            date: "2025-03-15".to_string(),
            tags: vec![
                "UI Design".to_string(),
                "CSS".to_string(),
                "React".to_string(),
                "Retro".to_string(),
            ],
            excerpt: "How to create nostalgic, terminal-inspired user interfaces while maintaining modern usability.".to_string(),
            content: r#"
# Building Retro-Tech UIs for Modern Applications

There's something nostalgic and appealing about the aesthetics of early computing - the monospaced fonts, the glowing terminal text, and the minimalist interfaces. In this post, I'll share techniques for creating retro-tech inspired UIs that still meet modern usability standards.

## Finding the Right Balance

The key to a successful retro-tech UI is finding the right balance between authentic vintage aesthetics and modern usability. You want to capture the feel of old-school technology without subjecting users to the actual limitations of those systems.

Consider:
- Using monospaced fonts for key UI elements, but not necessarily all text
- Incorporating scan lines or subtle CRT effects that don't interfere with readability
- Adding boot sequences and terminal-style interactions as enhancements, not requirements

## Color Palettes Matter

The right color palette is crucial for retro-tech UIs. Consider these popular options:

1. **Amber Terminal** - Black backgrounds with amber (#FFB000) text
2. **Green Screen** - Black backgrounds with green (#33FF33) text 
3. **IBM PC** - Black with light cyan and white
4. **TRON Legacy** - Black with cyan/blue glowing elements

## Technical Implementation

For web applications, a combination of CSS and JavaScript can achieve most retro effects:

```css
.terminal {
  font-family: 'VT323', 'Courier New', monospace;
  background-color: #000;
  color: #33FF33;
  text-shadow: 0 0 5px rgba(51, 255, 51, 0.8);
  padding: 2rem;
  border: 1px solid #33FF33;
  border-radius: 5px;
}
```

For scan lines:

```css
.scan-lines {
  background: linear-gradient(
    to bottom,
    rgba(255, 255, 255, 0),
    rgba(255, 255, 255, 0) 50%,
    rgba(0, 0, 0, 0.2) 50%,
    rgba(0, 0, 0, 0.2)
  );
  background-size: 100% 4px;
  height: 100%;
  width: 100%;
  position: absolute;
  pointer-events: none;
  opacity: 0.15;
}
```

## Accessibility Considerations

Always ensure your retro UI doesn't compromise accessibility:
- Maintain sufficient color contrast
- Ensure all interactive elements are keyboard accessible
- Provide alternative text for decorative elements
- Test with screen readers

## Conclusion

Retro-tech UIs can provide a unique and memorable experience for users when implemented thoughtfully. By balancing nostalgia with usability, you can create interfaces that stand out while still being functional and accessible.
            "#.to_string(),
        },
        Post {
            id: "post-3".to_string(),
            title: "My Journey Learning Systems Programming with Rust".to_string(),
            date: "2025-02-20".to_string(),
            tags: vec![
                "Rust".to_string(),
                "Systems Programming".to_string(),
                "Learning".to_string(),
            ],
            excerpt: "Personal reflections on transitioning from web development to systems programming using Rust.".to_string(),
            content: r#"
# My Journey Learning Systems Programming with Rust

After years of working primarily with JavaScript and web technologies, I decided to dive into systems programming with Rust. Here's what I've learned along the way, the challenges I've faced, and why I believe it's made me a better programmer overall.

## Why Rust?

When looking to expand my knowledge into systems programming, I considered C, C++, and Rust. I ultimately chose Rust for several reasons:

- Memory safety without garbage collection
- Modern language features like pattern matching and type inference
- Helpful compiler error messages
- Growing community and ecosystem
- Ability to use it for both systems and web programming

## The Learning Curve

Coming from JavaScript, the learning curve was steep. Some concepts that were particularly challenging:

1. **The Borrow Checker** - Understanding ownership, borrowing, and lifetimes took time and practice
2. **Thinking About Memory** - Explicitly considering how data is stored and moved
3. **Compile-Time vs. Runtime** - Shifting from dynamic to static typing required a different mindset
4. **No More Nulls** - Using `Option<T>` instead of null references

## Projects That Helped Me Learn

I found that building small, focused projects was the best way to learn Rust:

- Command-line utility for processing CSV files
- Custom HTTP server implementation
- Embedded programming on a Raspberry Pi
- WebAssembly module for use in a web application

## How Rust Changed My Thinking

Learning Rust has fundamentally changed how I approach programming, even when working with other languages:

- I'm more careful about data ownership and copying
- I think more deeply about error handling
- I appreciate the value of strong type systems
- I look for opportunities to leverage compile-time guarantees

## Resources That Helped Me

If you're considering learning Rust, here are some resources I found invaluable:

- "The Rust Programming Language" book (aka "The Book")
- "Rust by Example" online documentation
- The Rustlings course
- "Programming Rust" by Jim Blandy and Jason Orendorff
- The incredibly helpful Rust community on Reddit and Discord

## Conclusion

While the journey hasn't always been easy, learning Rust has been incredibly rewarding. It's opened up new possibilities in my career and made me a more thoughtful programmer. Whether you're coming from web development like me or another programming background, I highly recommend giving Rust a try.
            "#.to_string(),
        },
    ]
}
