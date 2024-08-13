web::scope("/auth")
                    .service(login)
                    .service(logout)
                    .service(fetch_user)
                    .service(get_user),
            )