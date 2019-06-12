# example-rocket-crud-rest-api
An example of building a simple REST API using Rocket, Diesel, Serde and MySQL. This sample just an application which allow you to CRUD user.

## Requirements

1. Rust and Cargo (1.37.0-nightly)

2. Diesel CLI (1.4.0)

3. MySQL (5.7.19)

## Setup

**1. Clone the repository**

```bash
git clone https://github.com/paimanbandi/example-rocket-crud-rest-api.git
```

**2. Edit file .env**
```
DATABASE_URL=mysql://root:bismillah@localhost/rocket_crud
```

**3. Run Database setup**
```bash
diesel setup
```

**4. Run the project**
```bash
cargo run
```

## Endpoints
    
    POST /user
    
    GET /users
    
    PUT /user/{id}
    
    DELETE /user/{id}
    
## Call the APIs via cURL
    
**1. Create user**
```bash
$ curl -X POST -H "Content-Type: application/json" -d '{"name":"paiman","email":"mail@paiman.id","address":"Klaten"}' http://localhost:8000/user
```
**2. Read users**
```bash
$ curl http://localhost:8000/users
```
**3. Update user**
```bash
$ curl -X PUT -H "Content-Type: application/json" -d '{"name":"Bilal","email":"bilal@example.com","address":"Bogor"}' http://localhost:8000/user/1
```
**4. Delete user**
```bash
$ curl -X DELETE 'http://localhost:8000/user/1'
```

## Get the tutorial

You can find the tutorial for this project in **Bahasa Indonesia** on my blog:

<http://paiman.id/2019/06/11/rust-membuat-rest-api-menggunakan-rocket-web-framework/>

