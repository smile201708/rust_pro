use std::collections::HashMap;

struct Student {
    id: u32,
    name: String,
    age: u32,
}

struct Course {
    id: u32,
    name: String,
}

struct Enrollment {
    student_id: u32,
    course_id: u32,
}

struct StudentManagementSystem {
    students: HashMap<u32, Student>,
    courses: HashMap<u32, Course>,
    enrollments: Vec<Enrollment>,
}

impl StudentManagementSystem {
    // 添加学生
    fn add_student(&mut self, student: Student) {
        self.students.insert(student.id, student);
    }

    // 获取学生
    fn get_student(&self, student_id: u32) -> Option<&Student> {
        self.students.get(&student_id)
    }
    fn update_student(&mut self, student_id: u32, new_student: Student) -> Option<Student> {
        self.students.insert(student_id, new_student)
    }

    fn delete_student(&mut self, student_id: u32) -> Option<Student> {
        self.students.remove(&student_id)
    }

    // 添加课程
    fn add_course(&mut self, course: Course) {
        self.courses.insert(course.id, course);
    }

    // 获取课程
    fn get_course(&self, course_id: u32) -> Option<&Course> {
        self.courses.get(&course_id)
    }

    // 学生选择课程
    fn enroll_student(&mut self, student_id: u32, course_id: u32) {
        let enrollment = Enrollment {
            student_id,
            course_id,
        };
        self.enrollments.push(enrollment);
    }

    // 获取学生所选的所有课程
    fn get_student_courses(&self, student_id: u32) -> Vec<&Course> {
        let mut student_courses = Vec::new();
        for enrollment in &self.enrollments {
            if enrollment.student_id == student_id {
                if let Some(course) = self.courses.get(&enrollment.course_id) {
                    student_courses.push(course);
                }
            }
        }
        student_courses
    }

    // 获取课程的所有学生
    fn get_course_students(&self, course_id: u32) -> Vec<&Student> {
        let mut course_students = Vec::new();
        for enrollment in &self.enrollments {
            if enrollment.course_id == course_id {
                if let Some(student) = self.students.get(&enrollment.student_id) {
                    course_students.push(student);
                }
            }
        }
        course_students
    }
}

fn main() {
    let mut sms = StudentManagementSystem {
        students: HashMap::new(),
        courses: HashMap::new(),
        enrollments: Vec::new(),
    };

    // 添加学生
    let student1 = Student {
        id: 1,
        name: String::from("smile"),
        age: 20,
    };
    let student2 = Student {
        id: 1,
        name: String::from("smile01"),
        age: 20,
    };
    sms.add_student(student1);

    sms.add_student(student2);

    if let Some(student) = sms.get_student(1) {
        println!("Student: {}, Age: {}", student.name, student.age);
    }

    let updated_student = Student {
        id: 1,
        name: String::from("smile1"),
        age: 21,
    };
    sms.update_student(1, updated_student);

    if let Some(student) = sms.delete_student(1) {
        println!("Deleted Student: {}", student.name);
    }

    // 添加课程
    let course1 = Course {
        id: 1,
        name: String::from("Math"),
    };
    let course2 = Course {
        id: 2,
        name: String::from("english"),
    };

    sms.add_course(course1);
    sms.add_course(course2);

    // 学生选择课程
    sms.enroll_student(2, 1);
    sms.enroll_student(2, 2);

    // 获取学生所选的所有课程
    let student_courses = sms.get_student_courses(2);
    for course in student_courses {
        println!("Student selected course: {}", course.name);
    }

    // 获取课程的所有学生
    let course_students = sms.get_course_students(1);
    for student in course_students {
        println!("Course enrolled student: {}", student.name);
    }
}



