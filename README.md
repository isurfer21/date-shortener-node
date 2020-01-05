# date-shortener-node
Node.js based utility to shorten (encode) the date and expand (decode) shortened date back to original date

## Introduction
Actually, I have developed a new way to encode/decode dates within 3 or 4 characters by using base-99 format. So here, we are going to learn about the logic behind this utility tool and reason of it's creation.

#### Why did I created it?
Actually, while sharing the files across organisation I would have to maintain version numbers in the filename but those versions won't tell anything about the last updation date until I check when was it last updated. So to maintain the updation dates in the filename, I had started adding timestamp in the filename which solved my issue but looks bad and it also increases the length of the filename. 

So I was searching ways to reduce the timestamp, at least the date portion to short string may be having only few characters but can tell the date. Initially, I got an idea of using base36 format instead of date and month because maximum date will be 31 which is less than 36; similary maximum month is 12 that is under 36 too. Well that solves my day-to-day problem but when I applied the same logic to years then it was limited to 36 years only. Then, I thought of using base64 format but that is also limited upto 64 years. 

Since I was looking for universal date conversion, so I would require something like base99 format. I looked over the Internet but I didn't found anything like that. So I made it by my own.

#### What is a base99 format?
In simple language, base99 format refers to series of 0-9 followed by small & capital A-Z characters and variations of vowel characters, i.e., `0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZàèìòùÀÈÌÒÙáéíóúÁÉÍÓÚâêîôûÂÊÔÛÎäëïöüÄËÏ`; where each character refers to the position index in series.

Using this format, any number in between 0-99 can be represented by a character positioned in base99 sequence 

e.g., `12 → c, 19 → j, 34 → y`

where, `c` is placed at 12th position in the sequence, `j` at 19th while `y` at 34th.

Now here is a way to use the same logic for date, so to do that let us take a date and encode it as

`15-8-2019 → 15.8.2019 → f.8.kj → f8kj`

Similarly, decode it back as

`f8kj → f.8.kj → 15.8.2019 → 15-8-2019`

## Install
Install the app using npm

```
$ npm install -g date-shortener-node
```

## Usage
Open the application in terminal & run the required commands as shown below

### Examples
Few sample usages are given below

```
$ ds -t
2ckj
$ ds -t -s
2-12-2019 -> 2.12.2019 -> 2.c.kj -> 2ckj
$ ds -d 2ckj -s
2ckj -> 2.c.kj -> 2.12.2019 -> 2-12-2019
$ ds -e 2/12/2019 -s
2-12-2019 -> 2.12.2019 -> 2.c.kj -> 2ckj
```

### Help
Find out all the available command options & flags 

```
$ ds -h
DATE SHORTENER
It is a tool to shorten (encode) the date and expand (decode) shortened date back to original date.

Usage: ds [options]

Options:
    -h, --help                 display the help menu
    -v, --version              display the application version
    -e, --encode DD-MM-YYYY    encode the provided date
    -d, --decode DMY           decode the provided code
    -t, --today                encode today's date
    -s, --steps                show with steps

Examples: 
 $ ds -v 
 $ ds -t 
 $ ds -t -s 
 $ ds -e 15/08/19 
 $ ds -e 15/08/2019 -s 
 $ ds -d f8j 
 $ ds -d f8kj -s 

```

### Version
See the currently available version

```
$ ds -v
DATE SHORTENER (Version 1.0.0)
Copyright (c) 2019 Abhishek Kumar.
Licensed under the MIT License.
```

## Development
To generate the build, run this command

```
$ npm run build
```

### Publish
Publish the latest build on npm

```
$ npm login
$ npm publish
```

## References
- [Encode or decode date in 3 characters](http://akzcool.blogspot.com/2019/10/encode-or-decode-in-3-characters.html)