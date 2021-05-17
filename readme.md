# Exclaim

A declarative/functional template language built with Rust.  
Full Unicode support.  
Zero dependencies (except for testing).  

## Version: Pre-release 1

## Info

### Repository Structure

- ```exclaim/```
  - The source code for the template engine.
- ```exclaim_bench/```
  - Performance tests

### Language

You can find a formal definition of the Exclaim Language here: [/exclaim-grammar](https://github.com/Jakob-Strobl/exclaim-grammar).  
During early releases of Exclaim, the grammar repo may be ahead, even, or behind the current implementation of the language.  

---

# Exclaim Guide

Exclaim is a template language. A template language combines templates (source files) with data to produce some desired output file type. A common use case is building HTML pages using templates to bind data into the final rendered HTML pages.

## Templates

Templates are essentially source code. Usually, a mix of Unicode Text and code.

## Data

Data is sourced from two locations:

1. Global Data
    - Data that exists outside of template files.
2. Template Data
    - Data that exists only inside template files.

## Blocks

The template engine processes template files by splitting the source into blocks. A block can be one of two variants: a text block or a code block.

### Text Blocks

Text blocks are plain Unicode text.

### Code Blocks

Code blocks contain a piece of code. All code is encased by matched double curly brackets: ```{{ <code> }}```.

The code inside a code block can differ depending on the context, but in general, they follow the following structure:

```{{ <action!> <statement> }}```

## Actions

Actions are how we define the purpose of a code block, and every code block starts with an action. Actions are similar to keywords in other languages but with some minor differences. An action is an all-lowercase word followed by an exclamation mark '!'.

Currently, there four types of actions:

- ```write!```
- ```let!```
- ```render!```
- ```!``` (End/Null Action)

### write! Block

Let's start with the most straightforward action, ```write!```.

A write action expects an expression inside the code block. This expression must be serializable; otherwise, Exclaim will panic.

Let's look at an example:  
**Input:** ```{{ write! "hello world!" }}```

The code block containing the ```write!``` action (a write! block for short) processes the expression. The expression in this example is a simple string literal ```"hello world!```. When this ```write!``` block is processed, the expression is serialized into the output.

**Output:** ```hello world!```

This doesn't look that useful now, but ```write!``` blocks are how you render any data into the final output file.

### let! Block

```let!``` blocks allow you to assign an expression/value to a variable. Variables defined by ```let!``` blocks at most only exist within the template file they are defined.  

Variable names only support alphabetic Unicode characters and underscore (_). Variables can't contain numbers nor start with an underscore. These rules will likely change in the future.  

Here's an example of using a ```let!``` block:

**Input:**  

```none
{{ let! name = "exclaim!" }}
This template was compiled with {{ write! name }} 
```

As you can see, we assign the string literal expression ```"exclaim!"``` to a variable named ```name```.
We then write the value of the name on the following line, which gives us:  

```none

This template was compiled with Exclaim!
```

If you didn't notice, the first line is just a new line. By default, Exclaim will preserve all whitespace. There is no way to change how whitespace is handled, but in the future, it will be implemented.

### render! Block

```render!``` blocks are how we iterate through data over the same piece of template code. ```render!``` is very similar to how for-each/for-in loops work in other languages.  

Instead of trying to explain this with fancy words, let's look at an example.

For this example, let's imagine a variable ```usernames``` that contains an array of usernames (just strings).

**usernames**: ```["test", "apple", "admin", "user"]```.

We can print all of the usernames like so:

**Input**:  

```none
{{ render! name : usernames }}
{{ write! name }}
{{!}}
```

In the ```render!``` block, we iterate of ```usernames```. For each iteration, we bind the value of the current item in the array to the variable ```name```.

There is a ```write!``` block and a mysterious ```{{!}}``` block at the end. This mysterious block is a closing block and is sometimes called an "end block" or "null block." This block marks the end of the ```render!``` block. Each ```render!``` block is required to be closed with a ```{{!}}```.

```render!``` blocks are a type of block known as **unclosed blocks**. Unclosed blocks are blocks that create a new scope nested inside the original scope.

The opposite of unclosed blocks is **enclosed blocks**. ```let!``` and ```write!``` are two examples of enclosed blocks. They do not create a new scope inside the template.

The mysterious ```{{!}}``` block is known as a **closing block**. The closing block *closes* any unclosed block.  

Here's a quick look at how blocks manipulate the scope:

```none
// Template Scope
{{ render! name : usernames }} // Creates a new scope with the variable 'name' defined inside the scope

// Render scope 

{{!}} // Closes scope
// Template Scope
```

Finally, in the end, we get the following output.

**Output**:

```none

test


apple


admin


user

```

Again the amount of whitespace is due to how Exclaim handles whitespace currently.

## Transformations

Transformations are built-in functions that allow you to take some data ```x``` and transform it into ```y```.

To use transformations, you start with the pipe symbol at the end of the expression you wish to transform.

It would look something like this:

**Input**: ```{{ write! "hello" | uppercase }}```

Here we have a write block that contains the literal expression ```"hello"``` with the ```uppercase``` transformation.  ```uppercase``` turns all alphabetic characters into uppercase characters. So when we execute the code block, the result is this:

**Output**: ```HELLO```

There are many types of transformations, and not all of them work on the same types of data. As transforms stabilize, there will be an easy way to see all kinds of transforms for all types of data.

## Patterns

Patterns are a particular way to declare more than one variable at the same time. They work similar to patterns in Rust but with fewer features.

Let's look at a good use of patterns:

**usernames**: ```["test", "apple", "admin", "user"]```.

```none
{{ render! (name, index) : usernames | enumerate }}
ID: {{ write! index }} | Name: {{ write! name }}
{{!}}
```

In the render block, we declare the pattern ```(name, index)``` and bind it to the expression ```usernames | enumerate```.

Let's first break down the right-hand expression ```usernames | enumerate```.
```usernames``` refers to a variable with the value of an array of four names (defined above). The transform ```enumerate``` takes in an array and binds each item in that array with their index. In short it returns an array of tuples that looks like this: ```[("test", 0), ("apple", 1), ("admin", 2), ("user", 3)]```.

The render! block iterates through each item (in this case, a 2-tuple) and for each iteration binds it to the pattern ```(name, index)```. You may notice this pattern matches the structure of the tuple in the array, and that's on purpose. This pattern destructures each tuple during the iteration and binds the values inside each tuple to ```name``` and ```index``` respectively. So, for the first iteration, ```name``` is set to ```"test"``` and ```index``` is set to ```0```.

You don't have to destructure tuples with patterns, but it is pretty convenient at times. You could bind the enumerated array with a single variable per iteration, but you would then need to use transformations to access the correct element in the tuple.

In the end, our output would look something like this:

```none

ID: 0 | Name: test


ID: 1 | Name: apple


ID: 2 | Name: admin


ID: 3 | Name: user

```

Again, that's a lot of whitespaces...

## Data types

The data types used at runtime are broken into two categories: scalar and compound.

Scalar types are simple and only hold one value. The following scalars are:

- Unsigned integer
- Signed integer
- Float (f64)
- String

Compound types hold one or more values. The following compound types are:

- Tuples (N-sized)
- Arrays
- Objects
