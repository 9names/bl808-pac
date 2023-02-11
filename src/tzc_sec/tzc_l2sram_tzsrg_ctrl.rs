#[doc = "Register `tzc_l2sram_tzsrg_ctrl` reader"]
pub struct R(crate::R<TZC_L2SRAM_TZSRG_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_L2SRAM_TZSRG_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZC_L2SRAM_TZSRG_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZC_L2SRAM_TZSRG_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tzc_l2sram_tzsrg_ctrl` writer"]
pub struct W(crate::W<TZC_L2SRAM_TZSRG_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZC_L2SRAM_TZSRG_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<TZC_L2SRAM_TZSRG_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TZC_L2SRAM_TZSRG_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tzc_l2sram_tzsrg_r0_id_en` reader - "]
pub type TZC_L2SRAM_TZSRG_R0_ID_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tzc_l2sram_tzsrg_r0_id_en` writer - "]
pub type TZC_L2SRAM_TZSRG_R0_ID_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TZC_L2SRAM_TZSRG_CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `tzc_l2sram_tzsrg_r1_id_en` reader - "]
pub type TZC_L2SRAM_TZSRG_R1_ID_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tzc_l2sram_tzsrg_r1_id_en` writer - "]
pub type TZC_L2SRAM_TZSRG_R1_ID_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TZC_L2SRAM_TZSRG_CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `tzc_l2sram_tzsrg_r2_id_en` reader - "]
pub type TZC_L2SRAM_TZSRG_R2_ID_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tzc_l2sram_tzsrg_r2_id_en` writer - "]
pub type TZC_L2SRAM_TZSRG_R2_ID_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TZC_L2SRAM_TZSRG_CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `reserved_6_15` reader - "]
pub type RESERVED_6_15_R = crate::FieldReader<u16, u16>;
#[doc = "Field `tzc_l2sram_tzsrg_r0_en` reader - "]
pub type TZC_L2SRAM_TZSRG_R0_EN_R = crate::BitReader<bool>;
#[doc = "Field `tzc_l2sram_tzsrg_r0_en` writer - "]
pub type TZC_L2SRAM_TZSRG_R0_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_L2SRAM_TZSRG_CTRL_SPEC, bool, O>;
#[doc = "Field `tzc_l2sram_tzsrg_r1_en` reader - "]
pub type TZC_L2SRAM_TZSRG_R1_EN_R = crate::BitReader<bool>;
#[doc = "Field `tzc_l2sram_tzsrg_r1_en` writer - "]
pub type TZC_L2SRAM_TZSRG_R1_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_L2SRAM_TZSRG_CTRL_SPEC, bool, O>;
#[doc = "Field `tzc_l2sram_tzsrg_r2_en` reader - "]
pub type TZC_L2SRAM_TZSRG_R2_EN_R = crate::BitReader<bool>;
#[doc = "Field `tzc_l2sram_tzsrg_r2_en` writer - "]
pub type TZC_L2SRAM_TZSRG_R2_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_L2SRAM_TZSRG_CTRL_SPEC, bool, O>;
#[doc = "Field `reserved_19_23` reader - "]
pub type RESERVED_19_23_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tzc_l2sram_tzsrg_r0_lock` reader - "]
pub type TZC_L2SRAM_TZSRG_R0_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_l2sram_tzsrg_r0_lock` writer - "]
pub type TZC_L2SRAM_TZSRG_R0_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_L2SRAM_TZSRG_CTRL_SPEC, bool, O>;
#[doc = "Field `tzc_l2sram_tzsrg_r1_lock` reader - "]
pub type TZC_L2SRAM_TZSRG_R1_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_l2sram_tzsrg_r1_lock` writer - "]
pub type TZC_L2SRAM_TZSRG_R1_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_L2SRAM_TZSRG_CTRL_SPEC, bool, O>;
#[doc = "Field `tzc_l2sram_tzsrg_r2_lock` reader - "]
pub type TZC_L2SRAM_TZSRG_R2_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_l2sram_tzsrg_r2_lock` writer - "]
pub type TZC_L2SRAM_TZSRG_R2_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_L2SRAM_TZSRG_CTRL_SPEC, bool, O>;
#[doc = "Field `reserved_27_31` reader - "]
pub type RESERVED_27_31_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn tzc_l2sram_tzsrg_r0_id_en(&self) -> TZC_L2SRAM_TZSRG_R0_ID_EN_R {
        TZC_L2SRAM_TZSRG_R0_ID_EN_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn tzc_l2sram_tzsrg_r1_id_en(&self) -> TZC_L2SRAM_TZSRG_R1_ID_EN_R {
        TZC_L2SRAM_TZSRG_R1_ID_EN_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn tzc_l2sram_tzsrg_r2_id_en(&self) -> TZC_L2SRAM_TZSRG_R2_ID_EN_R {
        TZC_L2SRAM_TZSRG_R2_ID_EN_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:15"]
    #[inline(always)]
    pub fn reserved_6_15(&self) -> RESERVED_6_15_R {
        RESERVED_6_15_R::new(((self.bits >> 6) & 0x03ff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tzc_l2sram_tzsrg_r0_en(&self) -> TZC_L2SRAM_TZSRG_R0_EN_R {
        TZC_L2SRAM_TZSRG_R0_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn tzc_l2sram_tzsrg_r1_en(&self) -> TZC_L2SRAM_TZSRG_R1_EN_R {
        TZC_L2SRAM_TZSRG_R1_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn tzc_l2sram_tzsrg_r2_en(&self) -> TZC_L2SRAM_TZSRG_R2_EN_R {
        TZC_L2SRAM_TZSRG_R2_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:23"]
    #[inline(always)]
    pub fn reserved_19_23(&self) -> RESERVED_19_23_R {
        RESERVED_19_23_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn tzc_l2sram_tzsrg_r0_lock(&self) -> TZC_L2SRAM_TZSRG_R0_LOCK_R {
        TZC_L2SRAM_TZSRG_R0_LOCK_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn tzc_l2sram_tzsrg_r1_lock(&self) -> TZC_L2SRAM_TZSRG_R1_LOCK_R {
        TZC_L2SRAM_TZSRG_R1_LOCK_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn tzc_l2sram_tzsrg_r2_lock(&self) -> TZC_L2SRAM_TZSRG_R2_LOCK_R {
        TZC_L2SRAM_TZSRG_R2_LOCK_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:31"]
    #[inline(always)]
    pub fn reserved_27_31(&self) -> RESERVED_27_31_R {
        RESERVED_27_31_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_l2sram_tzsrg_r0_id_en(&mut self) -> TZC_L2SRAM_TZSRG_R0_ID_EN_W<0> {
        TZC_L2SRAM_TZSRG_R0_ID_EN_W::new(self)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_l2sram_tzsrg_r1_id_en(&mut self) -> TZC_L2SRAM_TZSRG_R1_ID_EN_W<2> {
        TZC_L2SRAM_TZSRG_R1_ID_EN_W::new(self)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_l2sram_tzsrg_r2_id_en(&mut self) -> TZC_L2SRAM_TZSRG_R2_ID_EN_W<4> {
        TZC_L2SRAM_TZSRG_R2_ID_EN_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_l2sram_tzsrg_r0_en(&mut self) -> TZC_L2SRAM_TZSRG_R0_EN_W<16> {
        TZC_L2SRAM_TZSRG_R0_EN_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_l2sram_tzsrg_r1_en(&mut self) -> TZC_L2SRAM_TZSRG_R1_EN_W<17> {
        TZC_L2SRAM_TZSRG_R1_EN_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_l2sram_tzsrg_r2_en(&mut self) -> TZC_L2SRAM_TZSRG_R2_EN_W<18> {
        TZC_L2SRAM_TZSRG_R2_EN_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_l2sram_tzsrg_r0_lock(&mut self) -> TZC_L2SRAM_TZSRG_R0_LOCK_W<24> {
        TZC_L2SRAM_TZSRG_R0_LOCK_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_l2sram_tzsrg_r1_lock(&mut self) -> TZC_L2SRAM_TZSRG_R1_LOCK_W<25> {
        TZC_L2SRAM_TZSRG_R1_LOCK_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_l2sram_tzsrg_r2_lock(&mut self) -> TZC_L2SRAM_TZSRG_R2_LOCK_W<26> {
        TZC_L2SRAM_TZSRG_R2_LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "tzc_l2sram_tzsrg_ctrl\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_l2sram_tzsrg_ctrl](index.html) module"]
pub struct TZC_L2SRAM_TZSRG_CTRL_SPEC;
impl crate::RegisterSpec for TZC_L2SRAM_TZSRG_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tzc_l2sram_tzsrg_ctrl::R](R) reader structure"]
impl crate::Readable for TZC_L2SRAM_TZSRG_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tzc_l2sram_tzsrg_ctrl::W](W) writer structure"]
impl crate::Writable for TZC_L2SRAM_TZSRG_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tzc_l2sram_tzsrg_ctrl to value 0x3f"]
impl crate::Resettable for TZC_L2SRAM_TZSRG_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x3f;
}
