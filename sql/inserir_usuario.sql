insert into
    usuarios (nome, login, email, senha)
values (
        $1,
        $2,
        $3,
        $4
    ) returning id,
    login,
    email