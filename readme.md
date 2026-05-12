a. How much data your publisher program will send to the message broker in one run? 

Jawab: Publisher mengirim 5 data (sesuai jumlah baris p.publish_event di main.rs).

b. The url of: “amqp://guest:guest@localhost:5672” is the same as in the subscriber program, what does it mean?

Jawab: Artinya Publisher dan Subscriber menggunakan protokol AMQP dan terhubung ke alamat server yang sama (RabbitMQ) agar pesan dari Publisher bisa sampai ke Subscriber.

![RabbitMQ](rabbitmq.png)
![alt text](rabbitmq2.png)
![alt text](image.png)
![alt text](image-1.png)
![alt text](image-2.png)
