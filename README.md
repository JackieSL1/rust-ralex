## What is rust-relax?
rust-relax is a small command line relational algebra calculator written in Rust based off of [RelaX](https://dbis-uibk.github.io/relax/landing). Supported operations include:
* Table creation
* Selection (select)
* Projection (project)
* Inner join (join)
* Left outer join (leftJoin)
* Right outer join (rightJoin)
* Full outer join (fullJoin)
* Set Union (union)
* Set Intersection (intersect)
* Set Compliement (-)
* Set Division (/)
* Cartesian Product (*)

## How do I use it?
First, clone the repo onto your machine
```
$ git clone https://github.com/JackieSL1/rust-relax
```

Make sure you have Rust & Cargo installed. From inside rust-relax, run the project with cargo
```
$ cd rust-relax
$ cargo run
```
This will open up the prompt. Create a new table like this
```
> Employees = { id, name, salary
        1, "Dave", 1000
        2, "Gary", 2000
        3, "Mary", 1500
        }
```
And view the table by typing its name
```
> Employees
id, name, salary
1, Dave, 1000
2, Gary, 2000
3, Mary, 1500
```
Then, try out some operations on it
```
> project name Employees
name
Dave
Gary
Mary

> select salary > 1000 and salary < 2000 Employees
id, name, salary
3, Mary, 1500

> project name (select salary > 1500 Employees)
name
Gary

> quit
Exiting... Have a nice day!
```

Have fun!
