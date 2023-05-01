from time import sleep
from multiprocessing import Process, Queue

def Producer(queue):
    i = 0
    while 1:
        sleep(5)
        print("Send", i)
        queue.put(i)
        i += 1

def Consumer(queue):
    while 1:
        data = queue.get()
        sleep(6)
        print("Received", data)

if __name__ == '__main__':
    q = Queue()
    p = Process(target=Producer, args=(q,))
    p.start()

    p2 = Process(target=Consumer, args=(q,))
    p2.start()

    p.join()
    p2.join()
