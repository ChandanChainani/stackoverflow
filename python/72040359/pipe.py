from time import sleep
from multiprocessing import Process, Pipe

def Producer(sendPipe):
    i = 0
    while 1:
        sleep(5)
        print("Send", i)
        sendPipe.send(i)
        i += 1

def Consumer(recvPipe):
    while 1:
        data = recvPipe.recv()
        sleep(6)
        print("Received", data)

if __name__ == '__main__':
    recvPipe, sendPipe = Pipe(False)
    p = Process(target=Producer, args=(sendPipe,))
    p.start()
    p2 = Process(target=Consumer, args=(recvPipe,))
    p2.start()

    p.join()
    p2.join()
