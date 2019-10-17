# Du.rs
### just a new computer language project write in rust

### 1. VM
  - [ ] CPU
    - [ ] instruction
        - [x] hlt
        - [x] load
        - [x] arithmetic
            - [x] add
            - [x] sub
            - [x] mul
            - [x] div
            - [x] inc
            - [x] dec
        - [x] jump
            - [x] jmp
            - [x] jmpf
            - [x] jmpb
            - [x] jeq
            - [x] jl
            - [x] jg
        - [x] conditional
            - [x] eq
            - [x] lt
            - [x] gt
        - [x] logical
            - [x] and
            - [x] or
            - [x] xor
            - [x] not
        - [x] stack
            - [x] push
            - [x] pop
        - [x] function call
            - [x] call
            - [x] ret
        - [ ] mem
            - [ ] read
            - [ ] store
        - [ ] float
        - [ ] More...
    - [x] decode
    - [x] execute
    - [x] registers
    - [x] program
    - [x] pc
    - [x] sp
    - [x] bp
  - [ ] Mem
    - [ ] heap
        - [ ] malloc 
        - [ ] gc

### 2. REPL

  - [x] input
    - [x] .help
    - [x] .exit
    - [x] .clear
    - [ ] .load_elf
    - [ ] debug
        - [x] .history
        - [x] .program
        - [x] .registers
        - [ ] More...

### 3. Assembler 

  - [ ] lexer
    - [x] token
  - [ ] parser
    - [x] instruction
    - [x] label_declaration
    - [x] label_usage
    - [ ] directive
        - [x] .asciiz
          - [ ] Escape character
            - [ ] \n
        - [ ] .ascii
        - [x] .code
        - [x] .data
        - [ ] more
  - [ ] assembler
    - [ ] elf
        - [x] header
    - [ ] first pass
        - [ ] symbol table
            - [x] add
            - [x] get value
            - [ ] sort
            - [ ] more
    - [ ] second pass
        
### 3. Dulang
- [ ] EBNF
    - [ ] tokens
        - [ ] Grouping tokens
            - [ ] ( ) [ ] { }
        - [ ] Unary/binary operators
            - [ ] \+ - ! ~ & *
            - [ ] LSHIFT = '<<'
            - [ ] RSHIFT = '>>'
            - [ ] EQ = '=='
            - [ ] NOTEQ = '!='
            - [ ] LTEQ = '<='
            - [ ] RTEQ = '>='
            - [ ] AND = '&&'
            - [ ] OR = '||'
            - [ ] \+ - | ^ LSHIFT RSHIFT
            - [ ] \* / % &
            - [ ] EQ NOTEQ < LTEQ > RTEQ
            - [ ] AND
            - [ ] OR
            - [ ] ? :
        - [ ] Assignment operators
            - [ ] COLON_ASSIGN  = ':='
            - [ ] ADD_ASSIGN  = '+='
            - [ ] SUB_ASSIGN  = '-='
            - [ ] AND_ASSIGN = '&='
            - [ ] OR_ASSIGN  = '|='
            - [ ] XOR_ASSIGN  = '^='
            - [ ] LSHIFT_ASSIGN  = '<<='
            - [ ] RSHIFT_ASSIGN  = '>>='
            - [ ] MUL_ASSIGN  = '*='
            - [ ] DIV_ASSIGN  = '/='
            - [ ] MOD_ASSIGN  = '%='
            - [ ] =
            - [ ] INC = '++'
            - [ ] DEC = '--'
        - [ ] Names/literals
            - [ ] NAME = [a-zA-Z_][a-zA-Z0-9_]*
            - [ ] INT = [1-9][0-9]* | 0[xX][0-9a-fA-F]+ | 0[0-7]+ | 0[0bB][0-1]+
            - [ ] FLOAT = [0-9]*[.][0-9]*([eE][+-]?[0-9]+)?
            - [ ] CHAR = '\'' . '\''
            - [ ] STR = '"' [^"]* '"
    - [ ] grammer
        - [ ] type_list      
        - [ ] name_list
        - [ ] base_type
        - [ ] type
        - [ ] enum_item
        - [ ] enum_items
        - [ ] enum_decl
        - [ ] aggregate_field
        - [ ] aggregate_decl
        - [ ] var_decl
        - [ ] const_decl
        - [ ] typedef_decl
        - [ ] func_param
        - [ ] func_param_list
        - [ ] func_decl
        - [ ] decl
    - [ ] Statements
        - [ ] assign_op
        - [ ] switch_case
        - [ ] switch_block
        - [ ] stmt
        - [ ] type_spec
        - [ ] operand_expr
        - [ ] base_expr
        - [ ] unary_expr
        - [ ] mul_op
        - [ ] mul_expr   
        - [ ] add_op
        - [ ] add_expr
        - [ ] cmp_op
        - [ ] cmp_expr
        - [ ] and_expr
        - [ ] or_expr
        - [ ] ternary_expr
        - [ ] expr 
- [ ] Lexer
- [ ] Parser
- [ ] Interpreter
- [ ] Compiler
- [ ] JIT
