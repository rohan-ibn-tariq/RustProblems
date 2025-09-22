# Employee Department Manager

A Rust command-line application that manages employee assignments to company departments using HashMap and Vector data structures.

## Problem Definition

Create a text-based interface that allows users to:
- Add employees to specific departments using commands like "Add Sally to Engineering"
- Retrieve lists of employees by department (alphabetically sorted)
- Display all employees in the company organized by department (both departments and employees sorted alphabetically)

## Requirements

### Data Structure
```rust
use std::collections::HashMap;

// Core data structure
HashMap<String, Vec<String>>
```

- **Key**: Department name (e.g., "Engineering", "Sales")
- **Value**: Vector of employee names in that department

### Function Signatures
```rust
fn main() -> Result<(), Box<dyn std::error::Error>>
fn parse_command(input: &str) -> Command
fn add_employee(departments: &mut HashMap<String, Vec<String>>, name: String, department: String)
fn list_department(departments: &HashMap<String, Vec<String>>, department: &str)
fn list_all_departments(departments: &HashMap<String, Vec<String>>)
```

### Command Interface

#### Add Employee Command
- **Format**: `"Add [Name] to [Department]"`
- **Examples**: 
  - `"Add Sally to Engineering"`
  - `"Add Amir to Sales"`
  - `"Add Bob Smith to Marketing"`
- **Behavior**: Add employee to specified department, create department if it doesn't exist

#### List Department Command
- **Format**: `"List [Department]"`
- **Behavior**: Display all employees in specified department (alphabetically sorted)
- **Example**: `"List Engineering"` → Show all Engineering employees
- **Output format**:
  ```
  Engineering:
    Bob
    Sally
  ```

#### List All Command  
- **Format**: `"List all"`
- **Behavior**: Display all departments and their employees
  - Departments sorted alphabetically
  - Employees within each department sorted alphabetically
- **Output format**:
  ```
  Engineering:
    Bob
    Sally
  Sales:
    Amir
  ```

#### Exit Command
- **Format**: `"quit"` or `"exit"`
- **Behavior**: Terminate the program gracefully

### Program Flow
```rust
fn main() {
    let mut departments: HashMap<String, Vec<String>> = HashMap::new();
    
    loop {
        // 1. Display prompt
        // 2. Read user input
        // 3. Parse command
        // 4. Execute command
        // 5. Display result/error
        // 6. Continue or exit
    }
}
```

## Detailed Specifications

### Input Parsing Rules
1. **Add commands**: 
   - Pattern: "Add [Name] to [Department]"
   - Extract name (everything between "Add" and "to")
   - Extract department (everything after "to")
   - Handle multi-word names and departments
   
2. **List commands**: 
   - Pattern: "List [Department]" or "List all"
   - Extract department name or detect "all" keyword
   
3. **Case handling**: 
   - Command keywords ("Add", "List", "to", "all") should be case-insensitive
   - Employee names and department names preserve original case
   
4. **Whitespace handling**: 
   - Trim leading and trailing spaces from entire input
   - Normalize internal spaces (multiple spaces → single space)

### Employee Management Rules
1. **Adding employees**:
   - Create new department vector if department doesn't exist
   - Add employee name to department vector
   - Allow same employee name in different departments
   - **Prevent duplicate names within same department**
   - Store names exactly as entered (preserve capitalization)

2. **Listing employees**:
   - Sort employee names alphabetically (case-insensitive comparison)
   - Sort department names alphabetically when showing all
   - Display "No employees found" for empty departments
   - Handle non-existent departments gracefully

### String Processing Considerations
- Use `String` for owned data in HashMap
- Use `&str` for function parameters where possible
- Handle UTF-8 properly for international names
- Preserve original case for display while using case-insensitive sorting

### Error Handling
- **Invalid command format**: Display helpful error message with expected format
- **Empty employee names**: Reject and prompt for valid input
- **Empty department names**: Reject and prompt for valid input
- **Non-existent department**: For list commands, display "Department not found"
- **Malformed input**: Provide clear feedback on what went wrong

### Edge Cases to Handle
- Empty input (just whitespace)
- Commands with extra whitespace
- Names/departments with special characters
- Very long names or department names
- Input with only partial command (e.g., just "Add" or "List")
- Case variations in commands ("add", "ADD", "Add", "LIST", "list")

### Memory Management
- Use `HashMap::new()` for initialization
- Use `Vec::new()` or `vec![]` for department employee lists
- Properly handle string ownership (move vs borrow)
- Clean up empty departments (optional enhancement)

### Example Usage
```
Welcome to Employee Department Manager!
Enter 'quit' to exit.

Enter command: Add Sally to Engineering
✓ Added Sally to Engineering

Enter command: Add Amir to Sales  
✓ Added Amir to Sales

Enter command: Add Bob to Engineering
✓ Added Bob to Engineering

Enter command: Add Sally to Engineering
✗ Sally already exists in Engineering

Enter command: List Engineering
Engineering:
  Bob
  Sally

Enter command: List Marketing
✗ Department 'Marketing' not found

Enter command: List all
Engineering:
  Bob
  Sally
Sales:
  Amir

Enter command: list ALL
Engineering:
  Bob
  Sally
Sales:
  Amir

Enter command: Add John Doe to Human Resources
✓ Added John Doe to Human Resources

Enter command: quit
Goodbye!
```

## Test Cases to Verify
- **Basic functionality**:
  - `Add Sally to Engineering` → Success
  - `List Engineering` → Show Sally
  - `Add Bob to Engineering` → Success  
  - `List Engineering` → Show Bob, Sally (sorted)
  
- **Multiple departments**:
  - `Add Amir to Sales` → Success
  - `List all` → Show both departments sorted
  
- **Duplicate prevention**:
  - `Add Sally to Engineering` (again) → Error message
  
- **Case insensitivity**:
  - `add john to sales` → Should work
  - `LIST engineering` → Should work
  - `list ALL` → Should work
  
- **Edge cases**:
  - Empty input → Error message
  - `List NonExistent` → Department not found
  - `Add  to Engineering` → Invalid name error
  - `Add John to  ` → Invalid department error
  
- **Multi-word handling**:
  - `Add John Doe to Human Resources` → Success
  - `List Human Resources` → Show John Doe
  
- **Exit conditions**:
  - `quit` → Program exits
  - `exit` → Program exits (optional)
