<?php

namespace App\DataFixtures;

use App\Entity\User;
use Doctrine\Persistence\ObjectManager;
use Symfony\Component\PasswordHasher\Hasher\UserPasswordHasherInterface;

class UserFixtures extends AbstractFixtures
{
    private const int HOW_MANY_USERS = 10;
    private const string CREATION_MESSAGE = 'Creating';
    private const string PASSWORD = 'password';

    public function __construct(protected UserPasswordHasherInterface $userPasswordHasher)
    {
        parent::__construct();
    }

    public function load(ObjectManager $manager): void
    {
        for ($i = 0; $i < self::HOW_MANY_USERS; ++$i) {
            $user = new User();
            $user->setEmail($this->faker->email);
            $user->setPassword($this->userPasswordHasher->hashPassword($user, self::PASSWORD));
            $manager->persist($user);
        }

        echo sprintf('%s %d users', self::CREATION_MESSAGE, self::HOW_MANY_USERS);
        $manager->flush();
    }
}
