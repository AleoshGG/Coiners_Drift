<?php
    session_start();

    require_once 'includes/conexion.php';

    // Si existe la sesion el email dentro de la sesión id_usuario se realiza la consulta a la base de datos
    if(isset($_SESSION['name']['email'])){
        $email = $_SESSION['name']['email'];

        $sql = "SELECT * FROM user WHERE email = '$email'";
        $login = mysqli_query($db, $sql);

        $name = mysqli_fetch_assoc($login);
    }
?>

<!DOCTYPE html>
<html lang="es">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Formulario de registro y acceso</title>
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
    <link href="https://fonts.googleapis.com/css2?family=Amaranth&family=Permanent+Marker&display=swap" rel="stylesheet">

    <link rel="stylesheet" href="css/styles.css">
</head>
<body>
    <?php require_once 'includes/cabecera.php' ?>

    <!-- Mensaje que se muestra en caso de que exista la sesión -->
    <?php if(isset($_SESSION['name'])) : ?>
        <div class="lg-exito">Bienvenido 
        <span id="lg-correo"> <?= $_SESSION['name']['email'] ?> </span>
        Logeado con éxito!
        <a href="logout.php" id="logout">Cerrar sesión</a>
        </div>
    <?php endif; ?>

    <div class="tarjeta">
        <h1>Por favor, regístrate <br>o <br> accede con tu cuenta</h1>

    <div id="enlaces">
        <a href="login.php" id="btninicio">Login</a>
        <a href="singup.php" id="btnregsitro">Registrarse</a>
    </div>
    </div>
    
    <?php require_once 'includes/footer.php' ?>