<?php

$server = 'localhost:3306';
$username = 'root';
$password = '117092';
$database = 'coiners_drift';

$db = mysqli_connect($server, $username, $password, $database);

// Comprobacion de conexión satisfactoria o no
try{
    $con = new PDO("mysql:host=$server;dbname=$database;", $username, $password);
} catch(PDOException $e){
    die('Conexion fallida: '. $e->getMessage());

}

// Codificación de caracteres a uft8
mysqli_query($db, "SET NAMES 'utf8");