$ devinit add redis ->
get request to web service
{ "redis"{ "version": latest } }
latest is if didnt input version;

$ devinit add redis -v 1.0 
{ "redis" { "version": "1.0" } }

if web service recognize corectely it will
return necceasary data with struct Recipe

devinit list 
will show top 5 services

$devinit list
>PostgreSQL
Redis
Kafka
RabbitMQ
K8s
1..10 pages
Next Page

if user choose PostgreSQL
Service: PostgreSQL

1) PostgreSQL 18 (Latest)
2) PostgreSQL 17 (LTS)
3) PostgreSQL 16 
4) PostgreSQL 15 (Not Supported) 
5) PostgreSQL 14 

after choosing version user should apply
and enter Yes or No

after that cli send get request


at the end devinit will support github like
function
it will import all requirenments from repo like space.

example:
$devinit get https://devinit.com/gorkyi-chocolate/rust-backend-cli

and export all requirenments 