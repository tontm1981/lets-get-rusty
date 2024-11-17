// Error handling is hard and complicated by nature, because we have to deal with several different things
// We have to think in:
//  - Defining errors
//  - Propagating them
//  - Handling or Discard them
//  - Report them to end users and other developers

// There are 2 types of erros: Recoverable and Unrecoverable
// Unrecoverable errors are panics
// Recoverable errors implements the `Error` trait.

//  pub trait Error: Debug + Display {
//      fn source(&self) -> Option<&(dyn Error + 'static)> {
//          None
//      }
//  }

//  Implementing Error trait to our custom error types, achieves some points:
//      - Semantic mark types as errors
//      - Standardizes
//          - Checking the error source
//          - User racing report
//          - Dev facing report

//  Custom error types can be struct or enums
//  ```
//      struct ServerError {
//          status_code: u8,
//          bory: String,
//          source: Box<dyn Error>
//      }
//  ```

//  Using structs for custom error types is appropriate when we want to attach more context to our error and our error doesn't need to distinghuish between different error types
//  When we need to distinguish different error types, enums will be more appropriate. It's indicated when we want different behaviors, based on the variants

//  ```
//      enum APIError {
//          UserInputError(String),
//          InternalError(Box<dyn Error>)
//      }
//  ```
fn main() {
    
}
