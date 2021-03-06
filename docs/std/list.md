List
====

Enumerations
------------

### List

A linked list enumeration.

```clojure
(enum List
	(Cons x next)
	Nil)
```

#### Example

```clojure
(def foo (Cons 3 (Cons 4 (Cons 5 Nil))))
(def empty Nil)
```

Functions
---------

### `fill`

`fill :: (Any a) => a -> Integer -> List`

Fills a list with a Value.

#### Arguments

* `x :: a `: The value to fill with.
* `n :: Integer`: The amount of values to put in the list.

#### Example

```clojure
(def foo (fill "a" 5)) ;; (Cons "a" (Cons "a" (Cons "a" (Cons "a" (Cons "a" Nil)))))
```

### `car`

`car :: (Any a) => List -> Maybe a`

Returns `Just` the first element if the list is not empty, and `Nothing` if the list is empty.

#### Arguments

* `list :: List`: The list to get the first value of.

#### Example

```clojure
(def foo (Cons 3 (Cons 4 (Cons 5 Nil))))
(car foo) ;; (Just 3)
```

### `cdn`

`cdn :: List -> Maybe List`

Returns `Just` the list without its first element if it is not empty, and `Nothing` if the list is empty.

#### Arguments

* `list :: List`: The list to get the tail of.

#### Example

```clojure
(def foo (Cons 3 (Cons 4 (Cons 5 Nil))))
(cdn foo) ;; (Just (Cons 4 (Cons 5 Nil)))
```

### `range`

`range :: Integer -> Integer -> List`

Returns a List with the Integers going from `start` to `end`.

#### Arguments

* `start :: Integer`: The starting Integer.
* `end :: Integer`: The ending Integer.

#### Example

```clojure
(def foo (range 0 4)) ;; (Cons 0 (Cons 1 (Cons 2 (Cons 3 Nil))))
```

### `length`

`length :: List -> Integer`

Returns the length of a List.

#### Arguments

* `list :: List`: The list to get the length of.

#### Example

```clojure
(length (Cons 3 (Cons 4 (Cons 5 (Cons 6 Nil))))) ;; 4
```

### `empty?`

`empty? :: List -> Bool`

Returns True if `list` is `Nil`, False otherwise.

#### Arguments

* `list :: List`: The list to check.

#### Example

```clojure
(empty? Nil) ;; True
(empty? (Cons 4 Nil)) ;; False
```

### `map`

`map :: (Any a) => List -> (a -> a) -> List`

Processes each element of a list in a function and gets the result.

#### Arguments

* `list :: List`: The list to process
* `callback :: (Any a) => a -> a`: The callback to run on each element.

#### Example

```clojure
(def foo (Cons 5 (Cons 6 (Cons 7 (Cons 8 Nil)))))
(map foo (λ (x) (* x 2))) ;; (Cons 10 (Cons 12 (Cons 14 (Cons 16 Nil))))
```

### `foldl`

`foldl :: (Any a) => a -> (a -> a -> a) -> List a -> a`

Folds every element into an accumulator by applying an operation, returning the final result.

#### Arguments

* `acc :: (Any a) => a`: The accumulator, or starting value.
* `f :: (Any a) => a -> a -> a`: The folding function.
* `list :: (Any a) => List a`: The list to fold.

#### Example

```clojure
(def l (Cons 1 (Cons 2 (Cons 3 (Cons 4 Nil)))))
(foldl 0 (\ (x y) (+ x y)) l) ;; 10
```

### `push`

`push :: (Any a) => List a -> a -> List a`

Push an element to a list.

#### Example

```clojure
(push Nil 55) ;; (Cons 55 Nil)
```
