-- Dev Only - Comment out for keeping db between restart
drop database if exists app_db;
drop user if exists app_user;

-- Dev Only - For quick iteration
create user app_user password 'app_password';
create database app_db owner app_user encoding = 'UTF-8';
