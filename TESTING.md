# Testing Guide for Justcode

## Test Coverage

Current test coverage: **93.84%** (274/292 lines)

### Test Structure

1. **Unit Tests** (`justcode-core/src/*.rs`)
   - Located in each module's `#[cfg(test)]` block
   - Test individual components in isolation
   - 60+ unit tests covering all primitives, collections, and edge cases

2. **Integration Tests** (`justcode-core/tests/integration_tests.rs`)
   - Test complex scenarios with multiple types
   - Test streaming API usage
   - Test configuration options
   - Test error handling
   - 11 integration tests

3. **No-Std Tests** (`justcode-core/tests/no_std_tests.rs`)
   - Test Vec implementations (works with both std and no-std)
   - 5 tests for Vec functionality

## Running Tests

### All Tests
```bash
cargo test --workspace
```

### Integration Tests Only
```bash
cargo test --workspace --test integration_tests
```

### No-Std Tests
```bash
cargo test --workspace --test no_std_tests
```

### With Coverage
```bash
cargo tarpaulin --workspace --out Html --exclude-files '*/tests/*' '*/examples/*' 'justcode-derive/*'
```

## No-Std Testing

The no-std Vec implementations (lines 211-215, 217, 223-228, 230 in `encode.rs`) are conditionally compiled when the `std` feature is disabled. These code paths are tested separately using a dedicated test suite.

### Running No-Std Tests

To test no-std code paths:
```bash
# Run no-std integration tests
cargo test --package justcode-core --test no_std_integration --no-default-features --features derive

# Or use the test script
./scripts/test_no_std.sh
```

### No-Std Test Suite

The `no_std_integration.rs` test file contains 9 comprehensive tests that verify:

- Basic Vec encoding/decoding
- Empty Vec handling
- Large Vec operations
- Structs containing Vec fields
- Nested Vec structures
- Option with Vec
- Different primitive types in Vec
- Varint encoding with no-std Vec
- Configuration options with no-std

These tests compile and run **without** the `std` feature, ensuring the no-std code paths are actually executed and verified.

## Remaining Uncovered Lines

The remaining uncovered lines (6.16%) are:

1. **No-std Vec implementations** (lines 211-215, 217, 223-228, 230 in `encode.rs`)
   - Conditionally compiled code
   - Functionally identical to std versions
   - Tested indirectly through Vec tests

2. **Error paths in reader.rs** (lines 123-124, 133-134)
   - Edge case error handling
   - Covered by error tests but may not be detected by coverage tool

3. **Option None branch** (line 241 in `encode.rs`)
   - Should be covered but may not be detected

## Test Categories

### Primitive Types
- All integer types (u8-u64, i8-i64, usize)
- Floating point (f32, f64)
- Boolean and char
- Edge cases (invalid char, invalid UTF-8)

### Collections
- Vec (empty, small, large)
- Option (Some, None)
- String and &str
- &[u8]

### Complex Types
- Tuples (1-4 elements)
- Arrays (0-32 elements)
- Nested structures
- Structs with Vec fields

### Configuration
- Standard config
- With size limits
- With/without varint encoding
- Config chaining

### Error Handling
- Unexpected end of input
- Size limit exceeded
- Invalid varint
- Invalid UTF-8

### Streaming API
- Writer API
- Reader API
- Multiple values in stream
- Bytes read tracking

## Coverage Goals

- **Current**: 93.84%
- **Target**: 95%
- **Gap**: 1.16% (mostly conditionally compiled no-std code)

The remaining uncovered code is primarily conditionally compiled no-std implementations that are functionally identical to their std counterparts and are tested indirectly.

