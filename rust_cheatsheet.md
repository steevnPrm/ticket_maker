# 🦀 Rust — Fondamentaux Cheatsheet

---

## 📦 Variables & Types

### Déclaration

```rust
let x = 5;                // immutable (par défaut)
let mut y = 10;           // mutable
const MAX: u32 = 100_000; // constante (type obligatoire)

// Shadowing
let x = 5;
let x = x * 2;            // x = 10, nouveau binding
```

### Types scalaires

| Catégorie   | Types                                     |
|-------------|-------------------------------------------|
| Entiers signés | `i8` `i16` `i32` `i64` `i128` `isize` |
| Entiers non signés | `u8` `u16` `u32` `u64` `u128` `usize` |
| Flottants   | `f32` `f64` *(f64 par défaut)*            |
| Booléen     | `bool` → `true` / `false`                |
| Caractère   | `char` → `'A'` Unicode 4 octets           |

### Types composés

```rust
// Tuple
let t: (i32, f64, char) = (1, 2.0, 'z');
let (a, b, _) = t;  // destructuring
let first = t.0;    // accès par index

// Array (taille fixe, stack)
let arr: [i32; 3] = [1, 2, 3];
let zeros = [0; 5]; // [0, 0, 0, 0, 0]

// Slice (vue sur un tableau)
let slice: &[i32] = &arr[0..2];
```

---

## 🔑 Ownership — Les 3 règles

> ⚠️ **Chaque valeur a un propriétaire unique.** Quand le propriétaire sort du scope, la valeur est `drop`ée automatiquement — pas de GC, pas de `free` manuel.

1. Chaque valeur a **un seul owner**
2. Il ne peut y avoir **qu'un owner à la fois**
3. Quand l'owner sort du **scope**, la valeur est libérée

```rust
// Move (types heap comme String)
let s1 = String::from("hello");
let s2 = s1;            // move : s1 est invalide !
// println!("{}", s1);  // ← compile error

// Clone (copie profonde)
let s3 = s2.clone();

// Copy (types stack : i32, f64, bool, char…)
let x = 5;
let y = x;              // x toujours valide (Copy trait)
```

```rust
// Move dans une fonction
fn takes(s: String) { /* s droppé ici */ }

let s = String::from("hi");
takes(s);               // s invalide après l'appel

// Récupérer l'ownership en retournant la valeur
fn gives() -> String {
    String::from("returned")
}
```

---

## 🔗 References & Borrowing

> 💡 Emprunter sans transférer l'ownership.

### Règles des références

| Situation | Autorisé ? |
|-----------|-----------|
| Plusieurs `&T` (immuables) simultanées | ✅ Oui |
| Une seule `&mut T` (mutable) à la fois | ✅ Oui |
| `&T` et `&mut T` en même temps | ❌ Non |
| Deux `&mut T` simultanées | ❌ Non |

```rust
// Référence immuable
fn calculate_len(s: &String) -> usize {
    s.len()
}
let s = String::from("hello");
let len = calculate_len(&s); // s toujours valide

// Référence mutable
let mut s = String::from("hello");
let r = &mut s;
r.push_str(" world");
// let r2 = &s; ← compile error tant que r existe
```

### Slices de string

```rust
let s = String::from("hello world");
let hello: &str = &s[0..5];
let world: &str = &s[6..11];

fn first_word(s: &str) -> &str { // préférer &str à &String
    let bytes = s.as_bytes();
    for (i, &b) in bytes.iter().enumerate() {
        if b == b' ' { return &s[0..i]; }
    }
    &s[..]
}
```

---

## 🏗️ Structs

```rust
// Struct classique
struct User {
    username: String,
    email:    String,
    active:   bool,
}

let mut user = User {
    username: String::from("alice"),
    email:    String::from("alice@example.com"),
    active:   true,
};
user.email = String::from("new@example.com");

// Struct update syntax
let user2 = User {
    email: String::from("bob@example.com"),
    ..user  // copie les champs restants
};

// Tuple struct
struct Point(f64, f64, f64);
let origin = Point(0.0, 0.0, 0.0);

// Méthodes
impl User {
    fn new(username: &str, email: &str) -> Self { // constructeur
        Self {
            username: String::from(username),
            email:    String::from(email),
            active:   true,
        }
    }
    fn is_active(&self) -> bool { // méthode
        self.active
    }
}
```

---

## 🔀 Enums & Pattern Matching

```rust
enum Direction { North, South, East, West }

// Enum avec données
enum Message {
    Quit,
    Move  { x: i32, y: i32 },
    Write (String),
    Color (u8, u8, u8),
}

// Option<T> — pas de null en Rust !
let some_val: Option<i32> = Some(42);
let no_val:   Option<i32> = None;

// Result<T, E> — gestion d'erreur
let result: Result<i32, String> = Ok(10);
let err:    Result<i32, String> = Err(String::from("oops"));
```

```rust
// match exhaustif
let msg = Message::Move { x: 3, y: 7 };
match msg {
    Message::Quit              => println!("Quit"),
    Message::Move { x, y }    => println!("Move {x},{y}"),
    Message::Write(text)       => println!("Write: {text}"),
    Message::Color(r, g, b)   => println!("RGB({r},{g},{b})"),
}

// if let (quand un seul cas compte)
if let Some(val) = some_val {
    println!("Got: {val}");
}

// while let
let mut stack = vec![1, 2, 3];
while let Some(top) = stack.pop() {
    println!("{top}");
}
```

---

## 🔁 Contrôle de flux

```rust
// loop (infini, peut retourner une valeur)
let result = loop {
    counter += 1;
    if counter == 10 { break counter * 2; }
};

// while
while counter < 5 { counter += 1; }

// for … in (idiomatique en Rust)
for i in 0..5       { /* 0,1,2,3,4   */ }
for i in 0..=5      { /* 0,1,2,3,4,5 */ }
for item in &vec    { /* référence    */ }
for (i, v) in vec.iter().enumerate() { /* index + valeur */ }

// Labels de boucle
'outer: for x in 0..5 {
    for y in 0..5 {
        if x == y { break 'outer; }
    }
}
```

---

## 📚 Collections courantes

### Vec\<T\>

```rust
let mut v: Vec<i32> = Vec::new();
let v = vec![1, 2, 3];         // macro

v.push(4);
v.pop();                        // Option<T>
let third = &v[2];             // panic si hors bornes
let third = v.get(2);          // Option<&T>, safe
v.iter().map(|x| x * 2).collect::<Vec<_>>();
```

### HashMap\<K, V\>

```rust
use std::collections::HashMap;

let mut scores: HashMap<String, i32> = HashMap::new();
scores.insert(String::from("Alice"), 100);
scores.entry(String::from("Bob")).or_insert(50); // insert si absent

let score = scores.get("Alice");                 // Option<&i32>
for (key, val) in &scores { println!("{key}: {val}"); }
```

### String vs &str

| | `String` | `&str` |
|---|---|---|
| Ownership | ✅ owned | ❌ borrowed |
| Mutable | ✅ | ❌ |
| Heap | ✅ | stack / static |
| Usage | créer/modifier | lire |

```rust
let s: String = String::from("hello");
let sl: &str  = "world";              // string literal
let sl2: &str = &s[0..3];            // slice de String
```

---

## ⚠️ Gestion d'erreurs

```rust
use std::fs;
use std::io;

// ? opérateur — propagation automatique
fn read_file(path: &str) -> Result<String, io::Error> {
    let content = fs::read_to_string(path)?; // retourne l'erreur si Err
    Ok(content)
}

// unwrap / expect (panic si Err/None — à éviter en prod)
let val = some_result.unwrap();
let val = some_result.expect("message d'erreur clair");

// match sur Result
match fs::read_to_string("file.txt") {
    Ok(content)  => println!("{content}"),
    Err(e)       => eprintln!("Erreur : {e}"),
}

// unwrap_or, unwrap_or_else
let val = result.unwrap_or(0);
let val = result.unwrap_or_else(|_| default_value());
```

---

## 🧬 Traits & Génériques

```rust
// Définir un trait
trait Summary {
    fn summarize(&self) -> String;
    fn preview(&self) -> String {          // méthode par défaut
        format!("{}...", &self.summarize()[..20])
    }
}

// Implémenter un trait
struct Article { title: String, content: String }

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}: {}", self.title, self.content)
    }
}

// Génériques avec trait bounds
fn notify<T: Summary>(item: &T) {
    println!("{}", item.summarize());
}

// Syntaxe where (plus lisible)
fn notify<T>(item: &T) where T: Summary + Clone {
    println!("{}", item.summarize());
}

// Traits courants
// Clone, Copy, Debug, Display, PartialEq, Eq, Hash, Iterator
```

---

## ⏳ Lifetimes

```rust
// Le compilateur refuse les dangling references
// Les lifetimes rendent explicite la durée de vie des refs

// Annotation de lifetime
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// Lifetime dans une struct
struct Important<'a> {
    part: &'a str, // la struct ne peut pas outlive la ref
}

// 'static — dure toute la durée du programme
let s: &'static str = "Je vis pour toujours";
```

---

## 🛠️ Macros utiles

| Macro | Usage |
|-------|-------|
| `println!("{val}")` | Affichage avec retour à la ligne |
| `eprintln!("{e}")` | Affichage sur stderr |
| `format!("{a} + {b}")` | Crée une `String` |
| `vec![1, 2, 3]` | Crée un `Vec<T>` |
| `assert!(cond)` | Panic si faux |
| `assert_eq!(a, b)` | Panic si a ≠ b |
| `todo!()` / `unimplemented!()` | Placeholder |
| `dbg!(&val)` | Debug print + retourne la valeur |
| `panic!("msg")` | Arrêt immédiat du programme |

---

## ⚡ Closures & Itérateurs

```rust
// Closure
let add = |x: i32, y: i32| x + y;
let double = |x| x * 2;             // types inférés

// Capturer l'environnement
let offset = 10;
let add_offset = |x| x + offset;    // capture par référence

// move closure (ownership dans le thread)
let s = String::from("hello");
let greet = move || println!("{s}");

// Itérateurs (lazy)
let v = vec![1, 2, 3, 4, 5];

let sum: i32 = v.iter().sum();
let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
let evens: Vec<&i32> = v.iter().filter(|&&x| x % 2 == 0).collect();
let first_even = v.iter().find(|&&x| x % 2 == 0); // Option<&i32>

// Chaînage
let result: Vec<String> = v.iter()
    .filter(|&&x| x > 2)
    .map(|x| format!("val={x}"))
    .collect();
```

---

*Rust edition 2021 — `rustc` / `cargo` · [doc.rust-lang.org](https://doc.rust-lang.org)*
