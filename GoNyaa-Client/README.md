# GoNyaa前端页面

依赖管理工具使用yarn

## 安装依赖
```
yarn install
```

### 本地开发调试
```
yarn serve
```

### 打包静态文件
```
yarn build
```

之后将/dist下的所有内容拷贝至`GoNyaa-Server/src/main/resources/static`目录下，使用Maven打包即可

### Lints and fixes files

```
yarn lint
```

### Customize configuration
See [Configuration Reference](https://cli.vuejs.org/config/).
