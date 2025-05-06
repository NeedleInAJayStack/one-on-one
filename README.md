# One-on-one

This is an application that enables storing details about manager/employee one-on-ones.

## Features

- Allows manager and employee access
- Supports pre-planned topics from manager and employee, in meeting notes, and follow-up items
- Supports surveys to track question responses on an ongoing basis

## Structure

Entity Relationship Diagram:

```mermaid
erDiagram
    user ||--o{ one_on_one : manager_id
    user ||--o{ one_on_one : employee_id
    one_on_one ||--o{ meeting : one_on_one_id
    one_on_one ||--o{ survey : one_on_one_id
    survey ||--o{ survey_response : survey_id
```

Table Diagram:

```mermaid
classDiagram
    class user{
      int id
      varchar name
      varchar email
    }
    class one_on_one{
      int id
      int manager_id
      int employee_id
      datetime last_activity
    }
    class meeting{
      int id
      int one_on_one_id
      date date
      varchar manager_topics
      varchar employee_topics
      varchar notes
      varchar action_items
    }
    class survey{
      int id
      int one_on_one_id
      varchar question?
    }
    class survey_response{
      int id
      int survey_id
      date date
      varchar question?
    }
```
