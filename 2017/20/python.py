#!/usr/bin/env python3
import re
from sys import stdin

class Particle:
    def __init__(self, pid, pos, velocity, acceleration):
        self.destroyed = False
        self.pid = pid
        self.x, self.y, self.z = pos
        self.vx, self.vy, self.vz = velocity
        self.ax, self.ay, self.az = acceleration

    def __repr__(self):
        return f'{self.pid}: ' \
               f'p=<{self.x},{self.y},{self.z}>, ' \
               f'v=<{self.vx},{self.vy},{self.vz}>, ' \
               f'a=<{self.ax},{self.ay},{self.az}>'

def re_extract(match):
    return [int(x) for x in match.split(',')]

def step(particles):
    for particle in particles:
        particle.vx += particle.ax
        particle.vy += particle.ay
        particle.vz += particle.az
        particle.x += particle.vx
        particle.y += particle.vy
        particle.z += particle.vz
    for particle in (p for p in particles if not p.destroyed):
        collided = False
        for collision in (p for p in particles if p is not particle and p.x == particle.x and p.y == particle.y and p.z == particle.z):
            collision.destroyed = True
            collided = True
        if collided:
            particle.destroyed = True

def sorted_pids(particles):
    return [x.pid for x in sorted(
        particles, key=lambda x: abs(x.x) + abs(x.y) + abs(x.z))]

REGEX = re.compile('^p=<([^>]+)>, v=<([^>]+)>, a=<([^>]+)>$')
particles = []
particle_id = 0
for line in stdin:
    match = REGEX.match(line)
    particle = Particle(
            particle_id,
            re_extract(match[1]),
            re_extract(match[2]),
            re_extract(match[3]))
    particle_id += 1
    particles.append(particle)

sort_key = lambda x: abs(x.x) + abs(x.y) + abs(x.z)
prev_order = sorted_pids(particles)
step(particles)
order = sorted_pids(particles)
while prev_order != order:
    prev_order = order
    step(particles)
    order = sorted_pids(particles)
print(order[0], sum(1 for p in particles if not p.destroyed))
