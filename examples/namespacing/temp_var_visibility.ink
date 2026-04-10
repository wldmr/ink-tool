* [First Use] -> first_use
* [Definition] -> definition
* [Second Use] -> second_use

- (first_use) My name is {name}, I know about nothing.
- (definition) ~ temp name = "Hase"
- (second_use) My name is {name}, I know about nothing.



/* TEST

1: First Use
2: Definition
3: Second Use
?> 1
My name is 0, I know about nothing.
RUNTIME WARNING: 'temp_var_visibility.ink' line 5: Variable not found: 'name'. Using default value of 0 (false). This can happen with temporary variables if the declaration hasn't yet been hit. Globals are always given a default value on load if a value doesn't exist in the save state.
My name is Hase, I know about nothing.
*/

/* TEST

1: First Use
2: Definition
3: Second Use
?> 2
My name is Hase, I know about nothing.
*/

/* TEST

1: First Use
2: Definition
3: Second Use
?> 3
My name is 0, I know about nothing.
RUNTIME WARNING: 'temp_var_visibility.ink' line 7: Variable not found: 'name'. Using default value of 0 (false). This can happen with temporary variables if the declaration hasn't yet been hit. Globals are always given a default value on load if a value doesn't exist in the save state.
*/
