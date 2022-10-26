---
title: Java 日期 utils
date: 2018-01-01
layout: post
mathjax: true
category:
- Java
- Date
---
*Java >= 8 推荐使用　DateTimeFormatter*
总是可以使用更强大的`Joda-Time`

缺点都是 `date_format`是固定的，非常不灵话

````java
// ThreadLocalDateUtil.java
import java.text.*;
import java.util.Date;

public class ThreadLocalDateUtil {

    // "2017-12-20T11:20:47+0800"
    private static final String date_format = "yyyy-MM-dd'T'HH:mm:ssZ";
    private static ThreadLocal<DateFormat> threadLocal = new ThreadLocal<DateFormat>();

    public static DateFormat getDateFormat() {
        DateFormat df = threadLocal.get();

        if(df==null){
            df = new SimpleDateFormat(date_format);
            threadLocal.set(df);
        }

        return df;
    }

    public static String format(Date date) throws ParseException {
        return getDateFormat().format(date);
    }

    public static Date parse(String strDate) throws ParseException {
        return getDateFormat().parse(strDate);
    }
}
````

````java
// SyncDateUtil.java
import java.text.ParseException;
import java.text.SimpleDateFormat;
import java.util.Date;

public class SyncDateUtil {

    private static SimpleDateFormat sdf = new SimpleDateFormat("yyyy-MM-dd'T'HH:mm:ssZ");

    public static String format(Date date) throws ParseException {
        synchronized(sdf) {
            return sdf.format(date);
        }
    }

    public static Date parse(String strDate) throws ParseException {
        synchronized(sdf) {
            return sdf.parse(strDate);
        }
    }
}
````

````java
// ConcurrentDateFormatUtilTest.java

import java.text.*;
import java.util.concurrent.atomic.AtomicInteger;
import java.util.concurrent.ConcurrentHashMap;
import java.lang.reflect.Method;
import java.lang.reflect.InvocationTargetException;
import java.util.Date;
import java.io.*;
import java.math.*;
import java.net.*;
import java.nio.file.*;
import java.util.*;
import java.util.concurrent.*;
import java.util.function.*;
import java.util.prefs.*;
import java.util.regex.*;
import java.util.stream.*;

public class ConcurrentDateFormatUtilTest {
    static volatile AtomicInteger n;

    public static void run(Class<?> ConcurrentDateFormatUtil) throws ParseException, InterruptedException,
            NoSuchMethodException, IllegalAccessException, InvocationTargetException {
        Method dateParse = ConcurrentDateFormatUtil.getMethod("parse", String.class);
        Method dateFormat = ConcurrentDateFormatUtil.getMethod("format", Date.class);
        n = new AtomicInteger(-1);

        Set<String> dateSet = ConcurrentHashMap.newKeySet();
        Set<Integer> numberSet = ConcurrentHashMap.newKeySet();
        Date[] dates = new Date[100000];

        for (int i = 0; i < 100000; i++) {
            dates[i] = (Date)dateParse.invoke(null, i + 1000 + "-11-22T00:00:00+0800");
        }

        ExecutorService executorService = Executors.newFixedThreadPool(100);
        long t1=System.currentTimeMillis();

        for(int i=0;i<100000;i++){
            executorService.execute(new Runnable() {
                @Override
                public void run() {
                    int number = n.incrementAndGet();
                    String date;

                    try {
                        date = (String)dateFormat.invoke(null, dates[number]);
                    } catch (Exception ex) {
                        throw new RuntimeException(ex);
                    }

                    numberSet.add(number);
                    dateSet.add(date);
                    //System.out.println(number+" "+date);
                }
            });
        }

        Thread.sleep(500);
        executorService.shutdown();

        try {
            boolean loop = true;
            do {    //等待所有任务完成
                loop = !executorService.awaitTermination(2, TimeUnit.SECONDS);  //阻塞，直到线程池里所有任务结束
            } while(loop);
        } catch (InterruptedException e) {
            e.printStackTrace();
        }

        long t2=System.currentTimeMillis();

        System.out.println(dateSet.size());
        System.out.println(numberSet.size());

        System.out.printf("total %d ms\n", t2-t1);
    }

    public static void jsh(Class<?> ConcurrentDateFormatUtil) {
        try {
            run(ConcurrentDateFormatUtil);
        } catch (Exception ex) {
            System.out.println(ex.getCause());
        }
    }

    public static void main(String[] args) {
        jsh(ThreadLocalDateUtil.class);
        //jsh(SyncDateUtil.class);
    }
}

````

在 `ConcurrentDateFormatUtilTest` 中测试，未发现有什么性能差异．