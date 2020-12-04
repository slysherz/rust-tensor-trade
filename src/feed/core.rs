use std::collections::HashMap;

use crate::oms::exchanges::StreamLike;

pub fn default_name() -> String {
    "generic".to_string()
}

pub struct Constant<S: Clone> {
    value: S,
    name: String,
}

impl<S: Clone> Constant<S> {
    pub fn new(value: S) -> Constant<S> {
        Constant {
            name: default_name(),
            value: value,
        }
    }
}

impl<S: Clone> Iterator for Constant<S> {
    type Item = S;

    fn next(&mut self) -> Option<S> {
        Some(self.value.clone())
    }
}

impl<S: Clone> StreamLike for Constant<S> {
    fn rename(&mut self, name: String) {
        self.name = name;
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn value(&self) -> Option<Self::Item> {
        Some(self.value.clone())
    }
}

pub struct Stream<'a, S>
where
    S: Clone + 'a,
{
    it: Box<dyn Iterator<Item = S> + 'a>,
    name: String,
    value: Option<S>,
}

impl<'a, S: Clone> Stream<'a, S> {
    pub fn new<T>(it: T) -> Stream<'a, S>
    where
        T: Iterator<Item = S> + 'a,
    {
        Stream {
            name: default_name(),
            it: Box::new(it),
            value: None,
        }
    }

    pub fn source<T>(it: T) -> Stream<'a, S>
    where
        T: Iterator<Item = S> + 'a,
    {
        Stream {
            name: default_name(),
            it: Box::new(it),
            value: None,
        }
    }
}

impl<'a, S: Clone> Iterator for Stream<'a, S> {
    type Item = S;

    fn next(&mut self) -> Option<S> {
        self.value = self.it.next();
        self.value.clone()
    }
}

impl<'a, S: Clone> StreamLike for Stream<'a, S> {
    fn rename(&mut self, name: String) {
        self.name = name;
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn value(&self) -> Option<Self::Item> {
        self.value.clone()
    }
}

pub struct Group<'a, S>
where
    S: Clone,
{
    streams: HashMap<String, Box<dyn StreamLike<Item = S> + 'a>>,
    name: String,
    value: Option<HashMap<String, Option<S>>>,
}

impl<'a, S: Clone> Group<'a, S> {
    pub fn new<T>(streams: Vec<T>) -> Group<'a, S>
    where
        T: StreamLike<Item = S> + 'a,
    {
        let mut result: HashMap<String, Box<dyn StreamLike<Item = S>>> = HashMap::new();

        for stream in streams {
            result.insert(stream.name(), Box::new(stream));
        }

        Group {
            name: default_name(),
            streams: result,
            value: None,
        }
    }
}

impl<'a, S: Clone> Iterator for Group<'a, S> {
    type Item = HashMap<String, Option<S>>;

    fn next(&mut self) -> Option<HashMap<String, Option<S>>> {
        let mut result = HashMap::new();
        let mut is_empty = true;

        for (key, stream) in &mut self.streams {
            let value = stream.next().clone();

            if value.is_some() {
                is_empty = false;
            }

            result.insert(key.clone(), value);
        }

        self.value = match is_empty {
            true => None,
            false => Some(result),
        };

        self.value.clone()
    }
}

impl<'a, S: Clone> StreamLike for Group<'a, S> {
    fn rename(&mut self, name: String) {
        self.name = name;
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn value(&self) -> Option<Self::Item> {
        self.value.clone()
    }
}
