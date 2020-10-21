use std::collections::HashMap;

// Base user structure
struct User{
    id: i32,
    data: HashMap<String, String>
}

/* User::data keys

key - description - type - privledges needed - Properties
id - userID - int32 - [ALL] - PK
username - username of user - String - [userinfo]
pwd_hash - hash of password - str - [not-extern-accessable]
pwd_salt - salt for password - str - [not-extern-accessable]
first_name - First Name of user - String - [userinfo]
last_name - Last Name of user - String - [userinfo]
email - email adress of user - String - [sensitive_userinfo]

telnr - Phone Number - String - [sensitive_userinfo] - CAN BE NULL

dob - Date of Birth - Date - [sensitive_userinfo]

address - Address of user - String - [sensitive_userinfo]

postalcode - Postcode - String - [sensitive_userinfo]

City - City of user - String - [sensitive_userinfo]

Medical - Medical info - String - [sensitve_userinfo]

studentID - ID of linked student (see Student struct) - int32 - [userinfo] - CAN BE NULL
teacherID - ID of tutor link (see Tutor struct) - int32 - [userinfo] - CAN BE NULL

privledges - Bitfield for rights - int32 - [ALL] 

*/

struct Student{
    user: User,
    data: HashMap<String, String>
}

/* Student::data keys
key - description - type - privledges needed - Properties
id - Student ID - int32 - [ALL] - PK
classIds - All ID's where Student participates in - String - [userinfo, classinfo]
difficulty - Student current degree - String - [userinfo]
mentorIDs - All tutorID's for linking with a mentor - String - [userinfo]
profile - Profile - String - [userinfo]
scheduleref - Reference to personal schedule file path - String - [userinfo, fs]
gradesref - Path to grades file - String - [sensitive_userinfo, fs]
*/


struct Teacher{
    user: User,
    data: HashMap<String, String>
}


