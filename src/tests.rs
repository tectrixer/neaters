#[test]
pub fn create_neural_network() {
    use crate::NeuralNetwork;
    let _ = NeuralNetwork::with_size(1, 1);
    // TODO: add some kind of assertion here
}

#[test]
pub fn save_and_load_neural_network() {
    use crate::NeuralNetwork;
    use tempfile::tempdir;
    // creating two different networks
    let nn = NeuralNetwork::with_size(3, 2);
    let nn2 = NeuralNetwork::with_size(4, 3);
    // if test panicks here, then you dont have system write access
    let dir = tempdir().unwrap();

    // saving both networks as a binary
    let file_location = dir.path().join("test-load.nn");
    let file_location2 = dir.path().join("test-load2.nn");
    nn.save_as(file_location.as_path().to_str().unwrap());
    nn2.save_as(file_location2.as_path().to_str().unwrap());

    // binary of first network should be equal to binary data in first file
    assert_eq!(
        nn.as_byte_representation(),
        NeuralNetwork::load_bytes_from(file_location.as_path().to_str().unwrap())
    );
    // both files should contain different data
    assert_ne!(
        NeuralNetwork::load_bytes_from(file_location.as_path().to_str().unwrap()),
        NeuralNetwork::load_bytes_from(file_location2.as_path().to_str().unwrap())
    );

    // load both networks from their respective files
    let new_nn = NeuralNetwork::load_from(file_location.as_path().to_str().unwrap());
    let new_nn2 = NeuralNetwork::load_from(file_location2.as_path().to_str().unwrap());

    // they should equal themselves, but not the other network
    assert_eq!(nn, new_nn);
    assert_eq!(nn2, new_nn2);
    assert_ne!(nn, new_nn2);
    assert_ne!(nn2, new_nn);

    // if this fails, then the file could not be deleted correctly
    dir.close().unwrap();
}

#[test]
pub fn create_solver() {
    use crate::Solver;
    let _ = Solver::with_size(1, 1, 1);
    // TODO: add some kind of assertion here
}

#[test]
pub fn save_and_load_solver() {
    use crate::Solver;
    use tempfile::tempdir;
    // creating two different solvers
    let sv = Solver::with_size(3, 2, 2);
    let sv2 = Solver::with_size(4, 3, 3);
    // if test panicks here, then you dont have system write access
    let dir = tempdir().unwrap();

    // saving both solvers as a binary
    let file_location = dir.path().join("test-load.nn");
    let file_location2 = dir.path().join("test-load2.nn");
    sv.save_as(file_location.as_path().to_str().unwrap());
    sv2.save_as(file_location2.as_path().to_str().unwrap());

    // binary of first solver should be equal to binary data in first file
    assert_eq!(
        sv.as_byte_representation(),
        Solver::load_bytes_from(file_location.as_path().to_str().unwrap())
    );
    // both files should contain different data
    assert_ne!(
        Solver::load_bytes_from(file_location.as_path().to_str().unwrap()),
        Solver::load_bytes_from(file_location2.as_path().to_str().unwrap())
    );

    // load both solvers from their respective files
    let new_sv = Solver::load_from(file_location.as_path().to_str().unwrap());
    let new_sv2 = Solver::load_from(file_location2.as_path().to_str().unwrap());

    // they should equal themselves, but not the other solver
    assert_eq!(sv, new_sv);
    assert_eq!(sv2, new_sv2);
    assert_ne!(sv, new_sv2);
    assert_ne!(sv2, new_sv);

    // if this fails, then the file could not be deleted correctly
    dir.close().unwrap();
}
