# hyperULE Docs

## Intro

hyperULE is a small and simple language with basic features and C-like syntax.

The main goals are as following:

- Continue to learn the inner workings of a compiler, especially transpiling into a shitty, less powerful language.
- Learning basic compiler optimizations
- Continue learning Rust 🦀
- *Bring DX features to to ULE based HHR:*
  - Functions
  - Arrays
  - Structs
  - Type-Safety
- *Work around common but hard-to-debug issues in ULE:*
  - `StrToInt(derp + """)`
- *If successful:*
  - *Create backend for Zebra, Newland/Lua and Cognex?*
  
    *--> One code for every platform*


```rust
struct Gs1Rule
{          
    string Prefix;
    int Min;
    int Max;
}

struct Gs1Code
{
string Prefix;
string Code;
}

Gs1Rule[] gs1Rules = [          
    gs1Rule { "01", 12, 0 },
    gs1Rule { "02", 12, 16 },
    gs1Rule { "10", 12, 16 },
    gs1Rule { "11", 12, 0 }
];

int currentCharIndex = 0;          
string output = "";

entry    
{               
while (currentCharIndex < StrLen(OUT1Data))
{
    Gs1Code code = parseNextGs1Code();

    if (code == null)
       panic { handleDefaultError(); }
       
    output += code.Code + "\t";
}

if (StrLen(output) >= 1);
    OUT1Data = SubStr(output, 0, StrLen(output) - 1);
}

void handleDefaultError()
{            
    ErrorBeep();
}

string twoChars = "";                       
string threeChars = "";

Gs1Code parseNextGs1Code()
{                        
    bool ruleFound = false;   
    int ruleIndex = 0;
    
    twoChars = $SubStr(OUT1Data, currentCharIndex, 2);
    threeChars = $SubStr(OUT1Data, currentCharIndex, 3);

    Gs1Rule rule = $SelectFirstStruct(<x : gs1Rules> => { StartsWith(twoChars) || StartsWith(threeChars) });
  
    if (rule == null)
        return null;
}
```