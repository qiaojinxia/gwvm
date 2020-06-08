# gwvm
a jvm  written by rust

parse class command

.gwvm java.lang.Object


#### apoint jre
.gwvm -x "xxx/xxx" java.lang.Object

if don't apoint the jre runtime Path It will find in system env JAVA_HOME if Systemenv  not exists
Next , look for the current folder。

#### apoint classpath 
.gwvm -c "xxx/xxx" java.lang.Object