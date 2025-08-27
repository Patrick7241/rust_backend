CREATE TABLE users (
                       id BIGINT PRIMARY KEY AUTO_INCREMENT,
                       username VARCHAR(50) NOT NULL,
                       password VARCHAR(255) NOT NULL
);

-- 插入初始化数据
INSERT INTO users (username, password) VALUES
                                           ('admin', '$2b$12$TuLIBnkqAXpim4D5fAWiweyRNjHX7juch4mCPSEFr2JGY7mEpWvse'),
                                           ('test', '$2b$12$9TQlcpGBHdDhOgUEoJeP/en1oMHN4vlzDQ7kLMiknbybs9l7BL50K');