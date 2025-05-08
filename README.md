# 🦀 Advanced Programming - High Level Networking (gRPC)

**Nama**  : Daniel Liman <br>
**NPM**   : 2306220753 <br>
**Kelas** : Pemrograman Lanjut A


## Reflection

> What are the key differences between unary, server streaming, and bi-directional streaming RPC (Remote Procedure Call) methods, and in what scenarios would each be most suitable?

Perbedaan dari:

1. Unary RPC

    Pada Unary RPC, hanya ada request tunggal yang dapat dikirim oleh sisi klien ke sisi server, dan hanya ada response tunggal yang dapat diterima oleh sisi klien dari sisi server. Implementasi ini lebih simple dibandingkan dengan RPC lainnya, sehingga lebih cocok untuk diimplementasikan pada aplikasi yang sederhana. Contohnya pada operasi CRUD biasa maupun operasi yang memerlukan waktu yang cepat.

2. Server Streaming

    Pada Server Streaming RPC, klien dapat menerima serangkaian response lewat mengirim request tunggal dari server. Semua response yang dikirim dari server tersebut akan dibaca oleh klien sampai semuanya habis. RPC jenis ini cocok untuk skenario dimana data yang akan dikirim ke sisi klien berjumlah besar atau panjang. Contohnya pada saat memantau log server secara real-time maupun pada saat mengunduh file yang besar dan membaginya ke beberapa chunk yang lebih kecil.

3. Bi-Directional (Bidirectional) Streaming RPC

    Pada Bi-Directional (Bidirectional) Streaming RPC, sisi klien dan server sama-sama membuka stream yang dapat menerima dan mengirim pesan secara bersamaan. Kedua stream ini saling independent, sehingga setiap pesan bisa saling disisipkan dan tidak mengganggu proses pengiriman data yang dilakukan oleh target pengiriman pesan tersebut. Kasus atau skenario yang cocok untuk menggunakan jenis RPC ini adalah pada skenario yang perlu interaksi dari sisi klien dan server secara real-time, seperti chat system maupun platform yang mendukung aksi kolaborasi seperti Figma dan Google Docs.

<br>

> What are the potential security considerations involved in implementing a gRPC service in Rust, particularly regarding authentication, authorization, and data encryption?

Pada Authentication, perlu dipastikan bahwa klien (atau server) benar-benar memiliki identitas yang sesuai dengan apa yang tertera di request. Untuk menjamin hal tersebut, bisa digunakan tambahan keamanan seperti implementasi mutual TLS, Token-based (JWT, OAuth2) untuk autentikasi pada API endpoints, atau bisa memakai API Key yang menjaga ekslusifitas dari resource yang bisa digunakan.

Pada Authorization, perlu dipastikan bahwa klien yang sudah terautentikasi hanya bisa mengakses resources yang memang diperbolehkan untuknya. Perlu diingat juga bahwa akses terhadap beberapa bagian di aplikasi sebaiknya juga dibatasi secara minimal agar klien yang berbeda hanya bisa mengakses bagian yang mereka berhak untuk akses (*Principle of Least Privilage*). Implementasi yang bisa dilakukan adalah Role‑Based Access Control (RBAC), Attribute‑Based Access Control (ABAC), maupun Per-method ACLs.

<br>

> What are the potential challenges or issues that may arise when handling bidirectional streaming in Rust gRPC, especially in scenarios like chat applications?

Tantangan yang paling sering terjadi dalam menggunakan bidirectional streaming dengan Rust gRPC adalah manajemen dari alur data yang dikirim dan diterima dari sisi klien, seperti upaya mencegah adanya race condition dan sinkronisasi antara klien dan server saat mengirim dan menerima pesan. Selain itu, perlu juga untuk memperhatikan alokasi resource yang benar dan penambahan load balancer agar aplikasi dapat menangani lalu lintas data yang banyak dan besar.

<br>

> What are the advantages and disadvantages of using the `tokio_stream::wrappers::ReceiverStream` for streaming responses in Rust gRPC services?

Salah satu kelebihan dalam menggunakan `tokio_stream::wrappers::ReceiverStream` terletak pada kemampuannya dalam adaptabilitas `mpsc::Receiver` milik Tokio ke `Stream` untuk response gRPC dari server, karena `ReceiverStream` mengimplementasikan (`implement`) elemen `Stream` pada Rust sehingga tidak perlu menambahkan adapter lagi. Walaupun begitu, `ReceiverStream` memiliki kekurangan dimana pemakaiannya bisa memperlambat program dan butuh penanganan error yang ekstra dalam implementasinya.

<br>

> In what ways could the Rust gRPC code be structured to facilitate code reuse and modularity, promoting maintainability and extensibility over time?

Untuk mempermudah code reuse, modularitas kode, maintainability dan extensibility project yang menggunakan gRPC dari Rust, kita dapat:
- Mengimplementasikan abstraksi (dengan `trait`)
- Menerapkan Dependency Injection
- Melakukan generation proto sekali saja (pada `build.rs`)
- Memisahkan layer kode dengan jelas, seperti pemisahan folder untuk file `proto`, logic service, repository, dan sebagainya
- Mengimplementasikan berbagai tipe testing, seperti unit tests dan integration tests untuk menjaga maintainability

<br>

> In the MyPaymentService implementation, what additional steps might be necessary to handle more complex payment processing logic?

Implementasi lanjutan yang bisa dilakukan pada `MyPaymentService` adalah:
- Input Validation untuk format `user_id`, `amount` yang harus positif, dan lainnya.
- Implementasi mekanisme yang dapat mengatasi race condition dan load yang besar dalam proses payment, seperti menggunakan request payment yang terikat pada ID yang unik, atomic transaction pada tempat penyimpanan data, maupun penggunaan Server Streaming RPC untuk menangani request yang berjumlah banyak.
- Meningkatkan error handling se-spesifik mungkin untuk mencegah terjadinya error yang menghambat kelancaran proses pembayaran (payment).

<br>

> What impact does the adoption of gRPC as a communication protocol have on the overall architecture and design of distributed systems, particularly in terms of interoperability with other technologies and platforms?

Perubahan paling utama pada penggunaan gRPC adalah mengubah arsitektur kode menjadi berbasis pada contract-first dengan komunikasi HTTP/2, sehingga meningkatkan konsistensi kode dan mengurangi skema generalisasi berbasis struktur JSON (JSON-schema drift) antar service dengan bahasa pemrograman yang lain.

Selain itu, karena gRPC dapat mendukung penggunaan banyak bahasa (yang terhubung via stub yang dibuat di setiap bahasa), interoperabilitas antar service yang terpisah oleh bahasa pemrograman menjadi lebih mudah dan simple. Oleh karena itulah gRPC sebenarnya lebih diuntungkan dalam implementasi struktur aplikasi **microservices**.

<br>

> What are the advantages and disadvantages of using HTTP/2, the underlying protocol for gRPC, compared to HTTP/1.1 or HTTP/1.1 with WebSocket for REST APIs?

HTTP/2 membawa beberapa keunggulan dibanding HTTP/1.1 (dengan atau tanpa WebSocket), seperti support akan multiplexing sehingga permintaan dan respons bisa berjalan bersamaan dalam satu koneksi, framing biner yang lebih efisien, serta dukungan server-push untuk mengirim resource sebelum diminta. Hal ini juga menurunkan latensi dan penggunaan koneksi TLS, serta menghilangkan kebutuhan akan penggunaan WebSocket untuk komunikasi dua arah.

Walaupun begitu, implementasi HTTP/2 lebih kompleks karena memerlukan dukungan di proxy dan firewall, debugging framing biner yang kurang transparan, dan walau sudah diperbaiki, masih ada potensi head-of-line blocking pada level aliran. Jika kita memakai WebSocket di HTTP/1.1, aplikasi kita bisa jadi lebih sederhana dan mudah di-maintain untuk skenario real-time murni di lingkungan yang tidak mendukung HTTP/2 penuh.

<br>

> How does the request-response model of REST APIs contrast with the bidirectional streaming capabilities of gRPC in terms of real-time communication and responsiveness?

Model REST API menggunakan pola “request -> receive”, di mana sisi klien selalu harus memulai setiap panggilan dan menunggu balasan sebelum melanjutkan, sehingga transaksi data yang banyak dalam waktu real-time akan memerlukan polling berkala atau WebSocket terpisah, yang menambah kompleksitas dan latensi dari aplikasi.

Sebaliknya, gRPC dengan bidirectional streaming membuka saluran dua arah yang tetap terbuka, dimana klien dan server dapat saling mengirim pesan kapan saja tanpa perlu menunggu, sehingga update terbaru langsung disampaikan begitu siap, sehingga dapat menghasilkan komunikasi real-time yang lebih responsif dan efisien pada aplikasi kita.

<br>

> What are the implications of the schema-based approach of gRPC, using Protocol Buffers, compared to the more flexible, schema-less nature of JSON in REST API payloads?

Pendekatan gRPC dengan Protocol Buffers akan memaksa definisi tipe data dan struktur pesan di file `.proto`, sehingga menghasilkan serialisasi biner yang lebih ringkas, cepat, dan aman dari perubahan tak terduga. Kemudahan versioning lewat aturan penomoran field dan deteksi kesalahan di waktu kompilasi juga meningkatkan konsistensi antar service. 

Sebaliknya, payload JSON di REST API bersifat sangat fleksibel dimana lebih mudah ditambah atau dikurangi field tanpa perlu recompile—namun berisiko terjadinya drifting schema, kesalahan runtime karena tipe data tak terduga, overhead parsing string lebih besar, dan dokumentasi kontrak yang kurang terstandarisasi.
