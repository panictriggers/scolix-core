use std::collections::HashMap;

// Base user structure
struct User{
    id: i32,
    data: HashMap 
}

/* User::data keys

key - description - type - privledges needed - Properties
id - userID - int32 - [ALL] - PK
username - username of user - String - [userinfo]
pwd_hash - hash of password - str - [not-extern-accessable]
pwd_salt - salt for password - str - [not-extern-accessable]
first_name - First Name of user - String - [userinfo]
last_name - Last Name of user - String - [userinfo]
email - email adress of user - String - [userinfo]
dob - Date of Birth - Date - [userinfo]
studentID - ID of linked student (see Student struct) - int32 - [userinfo] - CAN BE NULL
tutorID - ID of tutor student (see Tutor struct) - int32 - [userinfo] - CAN BE NULL
*/

struct Student{
    id: i32,
    data: HashMap
}

/* Student::data keys
key - description - type - privledges needed - Properties
id - Student ID - int32 - [ALL] - PK
classIds - All ID's where Student participates in - String - [userinfo, classinfo]
difficulty - Student current degree - String - [userinfo]
mentorIDs - All tutorID's for linking with a mentor - [userinfo]
profile - Profile IDs () 


