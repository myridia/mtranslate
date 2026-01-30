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


### Maintenance Queries 
```
-- DELETE a
SELECT a.id ,s.id ,t.id 

FROM `a_source_target` AS a

LEFT JOIN sv AS s
ON a.source_id = s.id

LEFT JOIN da AS t 
ON a.target_id = t.id  

WHERE `source_name` = 'sv' 
AND `target_name` = 'da'
AND (t.id IS NULL
OR s.id IS NULL
);
```



