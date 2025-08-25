CREATE TABLE users (
                       id BIGINT PRIMARY KEY AUTO_INCREMENT,
                       username VARCHAR(50) NOT NULL,
                       password VARCHAR(255) NOT NULL
);

-- 插入初始化数据
INSERT INTO users (username, password) VALUES
                                           ('admin', 'admin'),
                                           ('test', '1');