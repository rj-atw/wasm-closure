# Idea

Simplify the problems of data engineering by making first class the contituent logic transformation that 
comprise the fundemental complexity of organizations. This toolkit seeks leverage [Wasm|wasm.org] to define
these first class transforms (need name: inni). 

The toolkit will seek to reduce the complexity of creating an inni, in the hopes that more natural
units of business can define them. It also seeks to create the framework structures that will natural
extend inni's allowing them to scale to peta-byte scale data systems like Spark or Big Query does today.
Whilst also keeping inni fuctional for website or other small scale interfaces like Restful microservices,
React.js apps or GraphQL based services.

## The concept of an inni 

1. Basic Map/Reduce transform
1. ETL transforms
1. Finite State Machines
2. ...

# Code Structure

The initial prototype code is broken down into two main pieces that represent how a simple inni, defined
as a transformation taking an array of numbers to a single value. Can be defined once and yet work seamlessly
in both a react.js app and a rust cli application. 


## Frontend Prototype

The react.js application is broken into a [frontend|...] and [backend|...] directories. The frontend code
is a SPA (single page application) based UI which can be used to both defined a new rust based inni, see
a list of already defined inni's and execute inni's against any input array. The backend directory contains
an nodejs based API which list, dispatches, or compiles and uploads inni's used by the UI. The code is 
deployed to a GKE cluster powering the website rjjr.tech.


## Backend Prototype
The cli application is currently host in https://github.com/rj-atw/wasm-closure. Just clone and compile the
rust application using ```cargo build```. Then run it by providing the path to an WASM file defining a prototype 
inni as the first argument then a comma seperated array of numbers for the input to the given inni.


## Structure of the Prototype Inni definition

today# Idea

Simplify the problems of data engineering by making first class the contituent logic transformation that 
comprise the fundemental complexity of organizations. This toolkit seeks leverage [Wasm|wasm.org] to define
these first class transforms (need name: inni). 

The toolkit will seek to reduce the complexity of creating an inni, in the hopes that more natural
units of business can define them. It also seeks to create the framework structures that will natural
extend inni's allowing them to scale to peta-byte scale data systems like Spark or Big Query does today.
Whilst also keeping inni fuctional for website or other small scale interfaces like Restful microservices,
React.js apps or GraphQL based services.

## The concept of an inni 

1. Basic Map/Reduce transform
1. ETL transforms
1. Finite State Machines
2. ...

# Code Structure

The initial prototype code is broken down into two main pieces that represent how a simple inni, defined
as a transformation taking an array of numbers to a single value. Can be defined once and yet work seamlessly
in both a react.js app and a rust cli application. 


## Frontend Prototype

The react.js application is broken into a [frontend|...] and [backend|...] directories. The frontend code
is a SPA (single page application) based UI which can be used to both defined a new rust based inni, see
a list of already defined inni's and execute inni's against any input array. The backend directory contains
an nodejs based API which list, dispatches, or compiles and uploads inni's used by the UI. The code is 
deployed to a GKE cluster powering the website rjjr.tech.


## Backend Prototype
The cli application is currently host in https://github.com/rj-atw/wasm-closure. Just clone and compile the
rust application using ```cargo build```. Then run it by providing the path to an WASM file defining a prototype 
inni as the first argument then a comma seperated array of numbers for the input to the given inni.


## Structure of the Prototype Inni definition

tba
