# Challenge 2 API Server

## The Challenge

We are buiding an api server that will take the provided `data.json` file in the root of the repo load it into memeory and serve that data via REST endpoints.

Upon start the service will load the json file and store it in memory. then be able to update delete, or add to it.

### The endpoints

- /tweets: 
  - Method: GET 
  - Status Code Success: 200 
  - Desc: get all the tweets
  - Payload:
  ```json
[
  {
    "id": 32,
    "name": "dolorem architecto ut pariatur quae qui suscipit",
    "email": "Maria@laurel.name",
    "body": "nihil ea itaque libero illo\nofficiis quo quo dicta inventore consequatur voluptas voluptatem\ncorporis sed necessitatibus velit tempore\nrerum velit et temporibus"
  }
]
  ```
- /tweets?size=10&from=1
  - Method: GET
  - Status Code Success: 200
  - Query String:
    - size: how many tweets to return
    - from: offset of tweets to return
  - Desc: Only get a selected amount of tweets starting with the from offset
    - from: 1 == array[0]
  - Payload: 
  ```json
[  
  {
    "id": 32,
    "name": "dolorem architecto ut pariatur quae qui suscipit",
    "email": "Maria@laurel.name",
    "body": "nihil ea itaque libero illo\nofficiis quo quo dicta inventore consequatur voluptas voluptatem\ncorporis sed necessitatibus velit tempore\nrerum velit et temporibus"
  }
]
  ```
- /tweets/{:id}
  - Method: GET
  - Status Code Success: 200
  - Params:
    - id: id to specific tweet
  - Desc: Get a single tweet by ID
  - Payload:
```json
  {
    "id": 32,
    "name": "dolorem architecto ut pariatur quae qui suscipit",
    "email": "Maria@laurel.name",
    "body": "nihil ea itaque libero illo\nofficiis quo quo dicta inventore consequatur voluptas voluptatem\ncorporis sed necessitatibus velit tempore\nrerum velit et temporibus"
  }
```
- /tweets
  - Method: POST
  - Status Code Success: 201
  - Desc: based on the
  - Input: 
```json
  {
    "name": "dolorem architecto ut pariatur quae qui suscipit",
    "email": "Maria@laurel.name",
    "body": "nihil ea itaque libero illo\nofficiis quo quo dicta inventore consequatur voluptas voluptatem\ncorporis sed necessitatibus velit tempore\nrerum velit et temporibus"
  }
```
  - Payload:
```json
  {
    "id": 33,
    "name": "dolorem architecto ut pariatur quae qui suscipit",
    "email": "Maria@laurel.name",
    "body": "nihil ea itaque libero illo\nofficiis quo quo dicta inventore consequatur voluptas voluptatem\ncorporis sed necessitatibus velit tempore\nrerum velit et temporibus"
  }
```
- /tweets/{:id}
  - Method: PUT
  - Status Code Success: 202
  - Desc: based on the  
  - Input:
```json
  {
    "id": 33,
    "name": "Changed",
    "email": "Maria@laurel.name",
    "body": "nihil ea itaque libero illo\nofficiis quo quo dicta inventore consequatur voluptas voluptatem\ncorporis sed necessitatibus velit tempore\nrerum velit et temporibus"
  }
```
  - Payload:
```json
  {
    "id": 33,
    "name": "Changed",
    "email": "Maria@laurel.name",
    "body": "nihil ea itaque libero illo\nofficiis quo quo dicta inventore consequatur voluptas voluptatem\ncorporis sed necessitatibus velit tempore\nrerum velit et temporibus"
  }
```
- /tweets/{:id}
  - Method: DELETE
  - Status Code Success: 202
  - Desc: based on the  
  - Payload:
```json
  {
    "id": 33,
    "name": "Changed",
    "email": "Maria@laurel.name",
    "body": "nihil ea itaque libero illo\nofficiis quo quo dicta inventore consequatur voluptas voluptatem\ncorporis sed necessitatibus velit tempore\nrerum velit et temporibus"
  }
```

## How to add your project to this challenge

- You can fork the repo and and do a pull request to this (upstream repo)
  - https://gist.github.com/Chaser324/ce0505fbed06b947d962
  - Please do not merge yourself
- Please use the following directory structure:
  - /<git root>/<github usernme>/<language name>/<project root>
    - git root: is the root dir of the cloned git repo
    - github username: so we know who submitted it
    - language name: name of your project language
      - I added this so you may try the challenge in multiple languages if you please
      - please keep the name simmple without any special characters
        - C# should be csharp and not c# or c-sharp
    - project root: is where your project lives
      - example: with rust i run `cargo new project --bin` the `src/ Cargo.toml Cargo.lock` should live in project root

