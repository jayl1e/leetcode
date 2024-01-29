#include <unordered_map>
using std::unordered_map;



class LRUCache {
private:
    struct LNode{
        int key;
        int val;
        LNode* prev;
        LNode* next;
    };
    unordered_map<int, LNode*> kp;
    LNode* head;
    LNode* tail;
    int capacity;
public:
    LRUCache(int capacity) {
        this->head = new LNode{
            .prev = nullptr,
            .next = nullptr,
        };
        this->tail = new LNode{
            .prev= this->head,
            .next= nullptr,
        };
        this->head->next=this->tail;
        this->capacity = capacity;
    }
    
    int get(int key) {
        auto iter = kp.find(key);
        if (iter == kp.end()){
            return -1;
        }
        auto p = iter->second;
        take_front(p);
        return p->val;
    }
    
    void put(int key, int value) {
        if (this->kp.find(key)!=kp.end()) {
            this->kp[key]->val = value;
            take_front(this->kp[key]);
        }else if (this->kp.size()<this->capacity){
            auto p = new LNode{
                .key=key,
                .val=value,
            };
            put_front(p);
            this->kp[key] = p;
        } else{
            auto p = this->tail->prev;
            take_front(p);
            this->kp.erase(p->key);
            p->key = key;
            p->val = value;
            this->kp[key] = p;
        }
    }

private:

    void put_front(LNode* p){
        p->next = this->head->next;
        p->next->prev = p;
        this->head->next = p;
        p->prev = this->head;
    }

    void take_front(LNode* p){
        p->prev->next = p->next;
        p->next->prev = p->prev;

        put_front(p);
    }
};

/**
 * Your LRUCache object will be instantiated and called as such:
 * LRUCache* obj = new LRUCache(capacity);
 * int param_1 = obj->get(key);
 * obj->put(key,value);
 */