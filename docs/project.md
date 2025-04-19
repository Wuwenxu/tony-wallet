# 项目记事本
今天是2025年4月19日
1、搭建web服务，满足基础能力，后续再补齐代码。满足最基本的、日志、权限、mysql、redis接口的打通
docker mysql 安装 https://blog.csdn.net/ITLBoy/article/details/142369827?utm_medium=distribute.pc_relevant.none-task-blog-2~default~baidujs_baidulandingword~default-9-142369827-blog-137876198.235^v43^pc_blog_bottom_relevance_base2&spm=1001.2101.3001.4242.6&utm_relevant_index=11






删除mac 本地mysql service 报这个错，安装server 链接不上。 不能使用localhost ，需要使用127.0.0.1
查看容器 ip
docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' some-mysql
Failed to run custom build command for 'mysqlclient-sys v0.4.5*



需要将env.exmaple  变为 env 文件

thread 'main' panicked at src/config/app.rs:12:78:

called `Result::unwrap()` on an `Err` value: MissingValue("name")