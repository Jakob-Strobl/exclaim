# Exclaim's Grammar

## Template Language

Exclaim doesn't seperate actions/semantics using different syntax structures as seen in other template languages.  
e.g. ```{% %}```, ```{{ }}```  
Instead, every feature of the exclaim template language is accessed using one syntax: ```{{ }}```.  
A ```{{ }}``` in exclaim is called a block.

## Blocks

Blocks are the heart and soul of Exclaim. Inside every block is an expression. Actions, such as assignment, that are normally defined as statements in other languages are still expressions in Exclaim. This difference can be weird at first, but also empowers the language.


```{{ $ACTION $EXPR }}```

```$ACTION -> label! | !```  
A single ```!``` can be read as the the null Action or Action Terminator.

```$EXPR -> label = label | label1 | ... | labelN```  
```$EXPR -> label | label1 | ... | labelN```  
```$EXPR -> label | label1 arg1, ..., argN | labelN```  
```$EXPR -> label | filter [self.type == "kitchen" && || "pantry"] | take 1, 1```  
```$EXPR -> label```  

The following are equivalent  
```{{ assign! albums = site.content | filter [self.type == "kitchen"] }}```  
```{{ assign! albums = filter [site.content.type == "kitchen"] }}```  


Rendering multiple iterators requires them to be the same size
```
{{ render! album : albums | take 1, song : songs | take 1 }} 
    <li>{{ print! album.title }}</li>
    <li>{{ print! song.title }}</li>
{{!}}
```

The action ```print!``` is completely optional. Idiomatic Exclaim would leave out the action (it would be implicit). The example is only to show you how the template engine compiles the language.  

function take will return an error if we try to take a size larger than actual size of the source iterator
