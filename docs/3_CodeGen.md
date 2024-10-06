# hyperULE CodeGen

#### hyperULE Output V1.0

This file describes the code generation of the hyperULE compiler.

The compiler has to solve multiple issues as the target language does not support basic features such as:

- Type safety
- Arrays
- Functions
- Structs
- Precise memory allocations

To solve these challenges we must implement some static analyzing and introduce a small "runtime" in our generated
code which can handle things such as function calls, a call stack and a simulated heap.

## Call Stack

To allow for function calls, we implement a simple call stack. To further simplify this process, we split function
calls and memory.

When calling functions we add following to the call stack:

- Passed function parameters
- Function index to be called

Lets take `Greet(string)` as an example.

Greet takes in a string which, when unoptimized, will be stored on the heap.
In this case, the caller will push 1 byte, which is a pointer to the heap, on the call stack followed by the function
index to be called.

| Position | Description    | Bytes |
|----------|----------------|-------|
| CS-1     | Parameter      | 1     |
| CS       | Function Index | 1     |

### Optimizations

#### Developer optimizations

Developers of hyerULE have some ways of optimizing the output ULE code. 

- **const Values** *(unimplemented yet)*
  
  Declaring fields as `const` allows the compiler to optimize the output size.

- **Inline functions** *(unimplemented yet)*

  Defining functions as `inline` functions tell the compiler to not create a separate routine in the output code, but
  instead inline the instructions within the caller function.

- Global variables

  Global variables are not defined on the stack and will be inlined by the compiler. This results in smaller and faster code with less memory usage. Parameters and local variables will be defined on the virtual runtime stack but are retained within the function scope. 

#### Compiler optimizations

- **Dead code elimination** (unimplemented yet)

  test

- **Unnecessary code elimination** (unimplemented yet)

  Since we have to do many stack operations

```js
string Greet(string name) {
    return "Hello " + name + " from HyperULE!";
}

entry {
    OUT1.Data = Greet(OUT1.Data);
}
```

Un-Optimized
```js
{
    callStack = "\x01" // contains the main stack
    dynStack = ""      // contains memory allocations for strings and arrays
    csSize = 1         // call stack size
    cfsSize = 1        // current function stack size (params, function pointer, local vars)
    
    csAppend = ""
    
    // Registers for optimization
    ri = 0 // return int
    rs = 0 // return string
    fc = 0 // function counter

    
    f1ss=1
    while (strlen(callStack) > 0) {
        fc = StrToInt(Right(callStack), 1)
        csSize = StrLen(callStack)

        if (fc == 1) { // entry part 1
            cfsSize = 1
            dynStack = dynStack + IntToStr(StrLen(OUT1.Data)) + OUT1.Data
            csAppend = "\x02" + IntToStr(dsSize);
        } else if (fc == 2) { // greeting
            cfsSize = 2
            // arg is a string, so we perform string load operations
            i1 = StrToInt(Mid(funcStack, csSize - 1, 1)) // get pointer to length of name on heap
            // i1 = 0
            // 0 H e l l o
            pname = Mid(dynStack, i1, Mid(funcStack, i1, 1))

            rs = "Hello " + pname + " from hyperULE"
            csAppend = "\x03";
        } else if (fc == 3) { // entry part 2
            OUT1.Data = rs
            callStack = Left(callStack, csSize - 1)
        }

        callStack = Left(callStack, csSize - ) + csAppend
        csAppend = ""
    }
}
```

Optimized (O2)
```js
{
    
    heapMem = "" // contains dynamic memory allocations for dynamic strings and arrays
    callStack = "\x00" // contains the stack
    fnMem = ""
    
    // Registers for optimization
    i1 = 0 // gp int
    i2 = 0 // gp int
    s1 = "" // gp string
    s2 = "" // gp string
    ri = 0 // return int
    rs = 0 // return string
    fc = 0 // function counter
    
    while (strlen(callStack) > 0) {
        fc = StrToInt(Right(funcStack), 1)
        csSize = StrLen(callStack)
        
        if (fc == 0) { // entry
            
        } else if (fc == 1) { // greeting
            // arg is string, so perform string load operations
            i1 = StrToInt(Mid(funcStack, 1)) // get arg from stack
            pname = 
        }
    }
}
```

JS Testing environment

```js
function FixLen(sourceString, width, padCharacter, position) {
    // Ensure the padCharacter is a single character
    if (padCharacter.length !== 1) {
        throw new Error("Pad character must be a single character");
    }

    // Ensure position is either 0 or 1
    if (position !== 0 && position !== 1) {
        throw new Error("Position must be 0 (left) or 1 (right)");
    }

    // If the source string is already the correct width, return it
    if (sourceString.length === width) {
        return sourceString;
    }

    // If the source string is longer than the width, truncate it
    if (sourceString.length > width) {
        return position === 0
            ? sourceString.slice(0, width)
            : sourceString.slice(-width);
    }

    // If the source string is shorter than the width, pad it
    const padLength = width - sourceString.length;
    const padding = padCharacter.repeat(padLength);

    return position === 0
        ? sourceString + padding
        : padding + sourceString;
}

function StrLen(str) { return str.length; }
function StrToInt(str) { return parseInt(str); }
function IntToStr(int) { return int.toString() }
function Left(str, len) { return str.substring(0, len); }
function Right(str, len) { return str.substring(str.length - len, len); }
function Mid(str, start, len) { return str.substring(start, len); }


window.onload = () => {
    let callStack = "000" // contains the main stack
    let dynStack = ""      // contains memory allocations for strings and arrays
    let csSize = 1         // call stack size
    let cfsSize = 1        // current function stack size (params, function pointer, local vars)

    let csAppend = ""

    // Registers for optimization
    let ri = 0 // return int
    let rs = 0 // return string
    let fc = 0 // function counter

    let out1 = "Milu"

    let f1ss=0
    while (StrLen(callStack) > 0) {
        fc = StrToInt(Right(callStack, 3))
        csSize = StrLen(callStack)
        let dsSize = StrLen(dynStack)
        if (fc == 0) { // entry part 1
            cfsSize = 1
            dynStack = dynStack + FixLen(IntToStr(out1), 3, "0", 0) + out1
            csAppend = "002";
        } else if (fc == 2) { // greeting
            cfsSize = 2
            // arg is a string, so we perform string load operations
            i1 = StrToInt(Mid(callStack, csSize - 1, 1)) // get pointer to length of name on heap
            // i1 = 0
            // 0 H e l l o
            let pname = Mid(dynStack, i1, Mid(funcStack, i1, 1))

            rs = "Hello " + pname + " from hyperULE"
            csAppend = "003"
        } else if (fc == 3) { // entry part 2
            out1 = rs
            callStack = Left(callStack, csSize - 1)
        }

        callStack = Left(callStack, csSize - 1) + csAppend
        csAppend = ""
    }

    alert(out1)
}
```
