# Exclaim

A declarative/functional template language built with rust.  
Full unicode support.  
Zero dependencies (except for testing).  

## Version: Pre-release 1

---

## Info

### Repository Structure

- ```exclaim/```
  - The source code for the template engine.
- ```exclaim_bench/```
  - Performance tests

### Language

You can find a formal definition of the Exclaim Language here: [/exclaim-grammar](https://github.com/Jakob-Strobl/exclaim-grammar).  
During early realeases of Exclaim, the grammar repo may be ahead, even, or behind the current implementation of the language.  

## Basics

Exclaim is a template language. A template language is a programming language that allows you to combine templates with data to produce a text file as output.

### Templates

Templates are essentially source code. Usually a mix of Unicode Text

### Data

Data is sourced from two locations:

1. Global Data
    - Data that exists outside of template files.
2. Template Data
    - Data that exists only inside template files.

### Blocks

The template engine processes template files by splitting the source into blocks. A block can be one of two variants: a text block or a code block.

### Text Blocks

Text blocks are plain unicode text.

### Code Blocks

Code blocks contain a piece of code. All code is encased by matched double curly brackets: ```{{ <code> }}```.

// TODO expain actions and statements

// TODO detail the 4 actions

// TODO explain expressions

// TODO explain transformations

// TODO explain runtime data types