create table Struct(
    Name text not null,
    Architecture integer not null,
    primary key(Name, Architecture)
) without rowid;

create table Field(
    Parent text not null,
    Name text not null,
    Sequence integer not null,
    Type text not null,
    primary key(Parent, Name),
    foreign key(Parent) references Struct(Name)
) without rowid;