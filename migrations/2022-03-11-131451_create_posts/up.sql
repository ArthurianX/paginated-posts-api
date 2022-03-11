CREATE TABLE posts (
                       id INTEGER PRIMARY KEY AUTOINCREMENT,
                       title VARCHAR NOT NULL,
                       video_url VARCHAR NOT NULL,
                       thumb_url VARCHAR NOT NULL,
                       post_link VARCHAR NOT NULL,
                       tags TEXT NOT NULL

);

INSERT INTO
    "posts"(title, video_url, thumb_url, post_link, tags)
VALUES
    ("Green phones, ranked", "", "https://cdn.vox-cdn.com/thumbor/X2sD5atfEFX2DDagMwGeZz07k3k=/0x0:2040x1360/1820x1213/filters:focal(857x517:1183x843):format(webp)/cdn.vox-cdn.com/uploads/chorus_image/image/70608446/ajohnson_220310_5073_0001.0.jpg", "https://www.theverge.com/2022/3/11/22970820/apple-iphone-13-pro-alpine-green-samsung-s22-compared-ranked", "apple, iphone, 13")
