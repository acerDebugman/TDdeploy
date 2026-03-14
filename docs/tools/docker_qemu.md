

```
# 1. 准备目录并确认 ISO 存在
mkdir -p ~/vm-data
cp ubuntu-22.04-live-server-amd64.iso ~/vm-data/

# 2. 运行容器（本地 ISO 方式）
docker run -d \
  --name ubuntu-vm \
  --rm \
  --privileged \
  --cap-add NET_ADMIN \
  -p 8006:8006 \
  -p 2222:22 \
  -v /home/algo/vm-data:/storage \
  -v /home/algo/vm-data/ubuntu-22.04-live-server-amd64.iso:/boot.iso \
  -e RAM_SIZE=32G \
  -e CPU_CORES=16 \
  -e DISK_SIZE=100G \
  --device /dev/kvm \
  qemux/qemu:latest
```

启动后，进入 http://localhost:8006/  进行系统安装！



注意：

1. vm-data 目录只能有一个 iso 文件

2. 在 docker 的容器里 端口转发：

   ```
   socat TCP-LISTEN:8889,fork TCP:172.30.0.3:8889
   ```

在 docker 的容器里  直接用 iptables 转发 ip 层级的包：

```
# 1. 配置 DNAT（目的地址转换）：将发往 172.17.0.3 的流量转发到 172.30.0.3
iptables -t nat -A PREROUTING -d 172.17.0.3 -j DNAT --to-destination 172.30.0.3

# 2. 配置 SNAT（源地址转换）：让回包能正确返回（如果两个 IP 在不同网段）
iptables -t nat -A POSTROUTING -s 172.30.0.3 -d 172.17.0.0/16 -j SNAT --to-source 172.17.0.3

# 3. 开启 IP 转发
sysctl -w net.ipv4.ip_forward=1
```

这样：访问 172.17.0.3  就等价于直接访问 172.30.0.3

并且可以使用 ssh 直接访问：

```
ssh zgc@172.17.0.3

ssh root@172.17.0.3
```

而不用走 localhost 的 2222, 不需要如此访问：

```
ssh ubuntu@localhost -p 2222
```

内部配置了 iptables 后，访问 docker 容器的 IP 就等价于访问内部 虚拟机的IP !! 这样操作就方便多了！







1. 网络问题，网络如何配置(下面的方式也没有生效, 不使用，直接使用 iptables 转发比较方便)

   ```
   # 1. 在宿主机创建 macvlan 网络
   docker network create -d macvlan \
     --subnet=172.30.1.0/24 \
     --gateway=172.30.1.1 \
     -o parent=enp0s31f6 \
     qemu-macvlan
   
   # 2. 启动容器使用 macvlan 网络
   docker run -d \
     --name ubuntu-vm \
     --rm \
     --privileged \
     --network qemu-macvlan \
     --ip 172.30.1.2 \
     -v /home/algo/vm-data:/storage \
     -v /home/algo/vm-data/ubuntu-22.04-live-server-amd64.iso:/boot.iso \
     -e RAM_SIZE=16G \
     -e CPU_CORES=8 \
     -e DISK_SIZE=20G \
     qemux/qemu:latest
   ```

   

2. 向虚拟机内传送文件：

```
scp -P2222 tdengine-tsdb-enterprise-3.4.0.8-linux-x64.tar.gz algo@localhost:~
```



修改用户名密码，可以 root 登录：

```
# 先以普通用户登录
ssh ubuntu@localhost -p 2222

# 设置 root 密码
sudo passwd root
# 输入两次密码

# 允许 root 密码登录（修改 SSH 配置）
sudo sed -i 's/#PermitRootLogin prohibit-password/PermitRootLogin yes/' /etc/ssh/sshd_config
sudo systemctl restart ssh
```





cound img:





skills:







## 装 windows

```
docker run -d \
  --name windows-vm \
  --rm \
  --privileged \
  --cap-add NET_ADMIN \
  -p 8006:8006 \
  -p 2222:22 \
  -v /home/algo/docker/vm/windows:/storage \
  -v /home/algo/docker/vm/windows/winserver_2019.iso:/boot.iso \
  -e RAM_SIZE=32G \
  -e CPU_CORES=20 \
  -e DISK_SIZE=100G \
  --device /dev/kvm \
  qemux/qemu:latest
```





