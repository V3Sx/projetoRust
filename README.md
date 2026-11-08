Projeto Colaborativo em Rust.

Menu interativo (via terminal) que possibilita manipular: criar, editar,pagar e listar cada uma das estruturas implementadas. 
O objetivo prático foi consolidar a utilização correta do Borrowing no Rust.

Codigo por: Victor Hugo e Nicolas

-------------------------------------

Simulador de Usuários, Grupos, Arquivos e Permissões em Rust

Este projeto é um programa de terminal escrito em Rust que simula, de um jeito simples, como um sistema de arquivos no estilo Unix organiza usuários, grupos, arquivos, diretórios e permissões.

 O trabalho serviu pra aprender na prática como essas peças se conectam: quem é dono de um arquivo, a qual grupo ele pertence, quem pode ler, escrever ou executar, e o que muda quando um usuário entra ou sai de um grupo. Tudo roda em memória, ou seja, os dados existem enquanto o programa está aberto e não são gravados em disco.

-------
 Como o código está organizado

Cada conceito tem o seu próprio arquivo:

Arquivo	Responsabilidade
main.rs	Ponto de entrada: cria as listas e chama o menu.

menu.rs	Todo o menu interativo no terminal.

usuario.rs	Struct Usuario (nome, UID e grupos a que pertence).

grupo.rs	Struct Grupo (nome, GID e UIDs dos membros).

arquivo.rs	Struct Arquivo (nome, tamanho, dono, grupo e permissões).

diretorio.rs	Struct Diretorio (nome, dono, permissões e arquivos dentro dele).

permissao.rs	Struct Permissao (leitura, escrita, execução) e a conversão para octal.
