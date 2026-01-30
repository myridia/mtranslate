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

```
DELETE a  FROM `a_source_target` AS a LEFT JOIN `sv` AS s ON a.source_id = s.id LEFT JOIN `{}` AS t ON a.target_id = t.id  WHERE `source_name` = 'sv' AND `target_name` = '{}' AND (t.id IS NULL OR s.id IS NULL );
```

```
DELETE t FROM `{}` as t WHERE text REGEXP '<[^>]+>';
```

```
af
am
ar
az
be
bg
bn
bs
ca
ceb
co
cs
cy
da
de
el
en
eo
es
et
eu
fa
fi
fr
fy
ga
gd
gl
gu
ha
haw
hi
hmn
hr
ht
hu
hy
ig
is
it
iw
ja
jw
ka
kk
km
kn
ko
ku
ky
la
lb
lo
lt
lv
mg
mi
mk
ml
mn
mr
ms
mt
my
ne
nl
no
ny
or
pa
pl
ps
pt
ro
ru
sd
si
sk
sl
sm
sn
so
sq
sr
st
su
sv
sw
ta
te
tg
th
tl
tr
uk
ur
uz
vi
xh
yi
yo
zh-CN
zh-TW
zu
```


