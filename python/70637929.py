from pydantic import StrictStr, BaseModel, ValidationError, validator
from typing import List, Union

class User(BaseModel):
    name: Union[List[List[StrictStr]], List[StrictStr], StrictStr]
    cond: Union[List[StrictStr], StrictStr]

    @validator('cond')
    def check_cond(cls, v):
        assert not v == True, 'List is Empty'
        if isinstance(v, str) == True:
            del_opr = "delete"
            msg = "Operator not allowed `%s`" % (v)
            if len(del_opr) == len(v):
                assert v == "delete", msg
            else:
                assert v == del_opr[:len(v)], msg
        return v

try:
    User(name="this", cond="delete") # OK
    User(name=["this"], cond="delete") # OK
    User(name=[["this"]], cond="delete") # OK
    User(name=[["foo", "bar"], ["hey", "hi"]], cond="delete") # OK
    User(name=[["foo", "bar"], ["hey", "hi"]], cond="d") # OK
    User(name=[["this", 1]]) # ERR
    User(name=[[1]], cond="delete") # ERR
    User(name=5, cond="delete") # ERR
    User(name=[["foo", "bar"], ["hey", "hi"]], cond=1) # Err
    User(name=[["foo", "bar"], ["hey", "hi"]], cond=[]) # Err
    User(name=[["foo", "bar"], ["hey", "hi"]], cond=[1]) # Err
    User(name=[["foo", "bar"], ["hey", "hi"]], cond=["delete"]) # Err
    User(name=[["foo", "bar"], ["hey", "hi"]], cond="done") # Err
except ValidationError as e:
    print(e)

from typing import TypeVar, Generic

T = TypeVar('T')

class Condition(Generic[T]):
    def __init__(self, value: T, operator=None, list_predicate=None):
        self.value = v
