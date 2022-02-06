pub fn run() {
    let arr1 = [1, 2, 3, 4];
    let arr2 = arr1;
    println!("Array pointer {:?}", (arr1, arr2));
    //Because array is a premitive datatype it automatically takes refference of arr1
    // in arr2. So both arr1 and arr2 has same value and they both points to the same location
    // in memory

    let vec1 = vec![1, 2, 3];
    let vec2 = vec1; // Here it is deleting the vec1 and storing it in vec2;

    let vec3 = vec![1, 2, 3];
    let vec4 = &vec3; // In this case both vec3 and vec4 will work because
                      // with & I am making the reference an assignment which is not by default
}
