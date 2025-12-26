# ![email_gateway](pages/public/img/logo.png) mtranslate
* Myridia's online translator service

### Usage 
* 105 mysql tables 


### Create Tables via https://textmaker.myridia.com
```
CREATE TABLE `{}` (`id` int(11) NOT NULL,`hash` varchar(16) NOT NULL DEFAULT '',`text` longtext NOT NULL DEFAULT '') ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_uca1400_ai_ci;
ALTER TABLE `{}` ADD PRIMARY KEY (`id`),ADD UNIQUE KEY `hash` (`hash`);
ALTER TABLE `{}` MODIFY `id` int(11) NOT NULL AUTO_INCREMENT;COMMIT;
```





